//! GGA_X_OL2 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ol2.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ol2_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_bb = f64x8::splat(param_bb);
    let param_cc = f64x8::splat(param_cc);
    let param_aa = f64x8::splat(param_aa);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_bb * v_sigma0;
            let t29 = v_rho0 * v_rho0;
            let t30 = (simd::cbrt(v_rho0));
            let t31 = t30 * t30;
            let t33 = f64x8::splat(1.0) / t31 / t29;
            let t36 = ((v_sigma0).sqrt());
            let t37 = param_cc * t36;
            let t39 = f64x8::splat(1.0) / t30 / v_rho0;
            let t40 = f64x8::splat(M_CBRT2);
            let t43 = f64x8::splat(4.0) * t36 * t39 + t40;
            let t44 = f64x8::splat(1.0) / t43;
            let t45 = t39 * t44;
            let t47 = param_aa + f64x8::splat(0.013888888888888888) * t28 * t33 + t37 * t45;
            let t51 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t47));
            let t52 = (v_rho1).simd_le(dens_threshold);
            let t53 = -t16;
            let t55 = ((t14).select(t11, (t10).select(t15, t53 * t7)));
            let t56 = f64x8::splat(1.0) + t55;
            let t57 = (t56).simd_le(zeta_threshold);
            let t58 = (simd::cbrt(t56));
            let t60 = ((t57).select(t22, t58 * t56));
            let t61 = t60 * t26;
            let t62 = param_bb * v_sigma2;
            let t63 = v_rho1 * v_rho1;
            let t64 = (simd::cbrt(v_rho1));
            let t65 = t64 * t64;
            let t67 = f64x8::splat(1.0) / t65 / t63;
            let t70 = ((v_sigma2).sqrt());
            let t71 = param_cc * t70;
            let t73 = f64x8::splat(1.0) / t64 / v_rho1;
            let t76 = f64x8::splat(4.0) * t70 * t73 + t40;
            let t77 = f64x8::splat(1.0) / t76;
            let t78 = t73 * t77;
            let t80 = param_aa + f64x8::splat(0.013888888888888888) * t62 * t67 + t71 * t78;
            let t84 = ((t52).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t61 * t80));
            let tzk0 = t51 + t84;
            acc_zk = tzk0;
            let t85 = t6 * t6;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t16 * t86;
            let t89 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t87)));
            let t92 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t89));
            let t93 = t92 * t26;
            let t97 = t26 * t26;
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t25 * t98;
            let t102 = t5 * t99 * t47 / f64x8::splat(8.0);
            let t103 = t29 * v_rho0;
            let t105 = f64x8::splat(1.0) / t31 / t103;
            let t110 = f64x8::splat(1.0) / t30 / t29 * t44;
            let t113 = param_cc * v_sigma0;
            let t114 = t43 * t43;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t105 * t115;
            let t119 = -f64x8::splat(0.037037037037037035) * t28 * t105 - f64x8::splat(4.0) / f64x8::splat(3.0) * t37 * t110 + f64x8::splat(16.0) / f64x8::splat(3.0) * t113 * t116;
            let t124 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t47 - t102 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t119));
            let t125 = t53 * t86;
            let t127 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t125)));
            let t130 = ((t57).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * t127));
            let t131 = t130 * t26;
            let t135 = t60 * t98;
            let t138 = t5 * t135 * t80 / f64x8::splat(8.0);
            let t140 = ((t52).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t131 * t80 - t138));
            let tvrho0 = t51 + t84 + t6 * (t124 + t140);
            acc_vrho_0 = tvrho0;
            let t144 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t87)));
            let t147 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t144));
            let t148 = t147 * t26;
            let t153 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t148 * t47 - t102));
            let t155 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t125)));
            let t158 = ((t57).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * t155));
            let t159 = t158 * t26;
            let t163 = t63 * v_rho1;
            let t165 = f64x8::splat(1.0) / t65 / t163;
            let t170 = f64x8::splat(1.0) / t64 / t63 * t77;
            let t173 = param_cc * v_sigma2;
            let t174 = t76 * t76;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t165 * t175;
            let t179 = -f64x8::splat(0.037037037037037035) * t62 * t165 - f64x8::splat(4.0) / f64x8::splat(3.0) * t71 * t170 + f64x8::splat(16.0) / f64x8::splat(3.0) * t173 * t176;
            let t184 = ((t52).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t80 - t138 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t61 * t179));
            let tvrho1 = t51 + t84 + t6 * (t153 + t184);
            acc_vrho_1 = tvrho1;
            let t189 = f64x8::splat(1.0) / t36;
            let t190 = param_cc * t189;
            let t196 = f64x8::splat(0.013888888888888888) * param_bb * t33 + t190 * t45 / f64x8::splat(2.0) - f64x8::splat(2.0) * param_cc * t33 * t115;
            let t200 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t196));
            let tvsigma0 = t6 * t200;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t203 = f64x8::splat(1.0) / t70;
            let t204 = param_cc * t203;
            let t210 = f64x8::splat(0.013888888888888888) * param_bb * t67 + t204 * t78 / f64x8::splat(2.0) - f64x8::splat(2.0) * param_cc * t67 * t175;
            let t214 = ((t52).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t61 * t210));
            let tvsigma2 = t6 * t214;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
