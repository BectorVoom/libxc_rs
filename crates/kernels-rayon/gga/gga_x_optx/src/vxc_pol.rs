//! GGA_X_OPTX vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_optx.c`
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
pub fn gga_x_optx_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_b: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_b = f64x8::splat(param_b);
    let param_a = f64x8::splat(param_a);
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
            let t28 = param_gamma * param_gamma;
            let t29 = param_b * t28;
            let t30 = v_sigma0 * v_sigma0;
            let t31 = v_rho0 * v_rho0;
            let t32 = t31 * t31;
            let t33 = t32 * v_rho0;
            let t34 = (simd::cbrt(v_rho0));
            let t36 = f64x8::splat(1.0) / t34 / t33;
            let t39 = t34 * t34;
            let t43 = f64x8::splat(1.0) + param_gamma * v_sigma0 / t39 / t31;
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = t29 * t30 * t36 * t45 + param_a;
            let t52 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t48));
            let t53 = (v_rho1).simd_le(dens_threshold);
            let t54 = -t16;
            let t56 = ((t14).select(t11, (t10).select(t15, t54 * t7)));
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(t57));
            let t61 = ((t58).select(t22, t59 * t57));
            let t62 = t61 * t26;
            let t63 = v_sigma2 * v_sigma2;
            let t64 = v_rho1 * v_rho1;
            let t65 = t64 * t64;
            let t66 = t65 * v_rho1;
            let t67 = (simd::cbrt(v_rho1));
            let t69 = f64x8::splat(1.0) / t67 / t66;
            let t72 = t67 * t67;
            let t76 = f64x8::splat(1.0) + param_gamma * v_sigma2 / t72 / t64;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) / t77;
            let t81 = t29 * t63 * t69 * t78 + param_a;
            let t85 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t81));
            let tzk0 = t52 + t85;
            acc_zk = tzk0;
            let t86 = t6 * t6;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t16 * t87;
            let t90 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t88)));
            let t93 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t90));
            let t94 = t93 * t26;
            let t98 = t26 * t26;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t25 * t99;
            let t103 = t5 * t100 * t48 / f64x8::splat(8.0);
            let t104 = t32 * t31;
            let t106 = f64x8::splat(1.0) / t34 / t104;
            let t111 = param_b * t28 * param_gamma;
            let t112 = t30 * v_sigma0;
            let t113 = t32 * t32;
            let t114 = t113 * v_rho0;
            let t115 = f64x8::splat(1.0) / t114;
            let t118 = f64x8::splat(1.0) / t44 / t43;
            let t122 = -f64x8::splat(16.0) / f64x8::splat(3.0) * t29 * t30 * t106 * t45 + f64x8::splat(16.0) / f64x8::splat(3.0) * t111 * t112 * t115 * t118;
            let t127 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t94 * t48 - t103 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t122));
            let t128 = t54 * t87;
            let t130 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t128)));
            let t133 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t130));
            let t134 = t133 * t26;
            let t138 = t61 * t99;
            let t141 = t5 * t138 * t81 / f64x8::splat(8.0);
            let t143 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t134 * t81 - t141));
            let tvrho0 = t52 + t85 + t6 * (t127 + t143);
            acc_vrho_0 = tvrho0;
            let t147 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t88)));
            let t150 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t147));
            let t151 = t150 * t26;
            let t156 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t151 * t48 - t103));
            let t158 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t128)));
            let t161 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t158));
            let t162 = t161 * t26;
            let t166 = t65 * t64;
            let t168 = f64x8::splat(1.0) / t67 / t166;
            let t172 = t63 * v_sigma2;
            let t173 = t65 * t65;
            let t174 = t173 * v_rho1;
            let t175 = f64x8::splat(1.0) / t174;
            let t178 = f64x8::splat(1.0) / t77 / t76;
            let t182 = f64x8::splat(16.0) / f64x8::splat(3.0) * t111 * t172 * t175 * t178 - f64x8::splat(16.0) / f64x8::splat(3.0) * t29 * t63 * t168 * t78;
            let t187 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t162 * t81 - t141 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t182));
            let tvrho1 = t52 + t85 + t6 * (t156 + t187);
            acc_vrho_1 = tvrho1;
            let t193 = f64x8::splat(1.0) / t113;
            let t198 = -f64x8::splat(2.0) * t111 * t30 * t193 * t118 + f64x8::splat(2.0) * t29 * v_sigma0 * t36 * t45;
            let t202 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t198));
            let tvsigma0 = t6 * t202;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t206 = f64x8::splat(1.0) / t173;
            let t211 = -f64x8::splat(2.0) * t111 * t63 * t206 * t178 + f64x8::splat(2.0) * t29 * v_sigma2 * t69 * t78;
            let t215 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t211));
            let tvsigma2 = t6 * t215;
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
