//! GGA_K_RATIONAL_P vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`
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
pub fn gga_k_rational_p_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_p: f64,
    param_C2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_p = f64x8::splat(param_p);
    let param_C2 = f64x8::splat(param_C2);
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(1.0) / param_p;
            let t34 = f64x8::splat(M_CBRT6);
            let t35 = param_C2 * t32 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t41 = v_rho0 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(1.0) + t35 * t39 * v_sigma0 * t45 / f64x8::splat(24.0);
            let t50 = (simd::pow(t49, -param_p));
            let t51 = t31 * t50;
            let t52 = t6 * t51;
            let t54 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t52));
            let t55 = (v_rho1).simd_le(dens_threshold);
            let t56 = -t17;
            let t58 = ((t15).select(t12, (t11).select(t16, t56 * t8)));
            let t59 = f64x8::splat(1.0) + t58;
            let t60 = (t59).simd_le(zeta_threshold);
            let t61 = (simd::cbrt(t59));
            let t62 = t61 * t61;
            let t64 = ((t60).select(t24, t62 * t59));
            let t65 = t64 * t30;
            let t67 = v_rho1 * v_rho1;
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t68 * t68;
            let t71 = f64x8::splat(1.0) / t69 / t67;
            let t75 = f64x8::splat(1.0) + t35 * t39 * v_sigma2 * t71 / f64x8::splat(24.0);
            let t76 = (simd::pow(t75, -param_p));
            let t77 = t65 * t76;
            let t78 = t6 * t77;
            let t80 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t78));
            let tzk0 = t54 + t80;
            acc_zk = tzk0;
            let t81 = t7 * t7;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t17 * t82;
            let t85 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t83)));
            let t88 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t85));
            let t89 = t88 * t30;
            let t90 = t89 * t50;
            let t91 = t6 * t90;
            let t93 = f64x8::splat(1.0) / t29;
            let t94 = t28 * t93;
            let t95 = t94 * t50;
            let t96 = t6 * t95;
            let t97 = t96 / f64x8::splat(10.0);
            let t98 = param_C2 * t34;
            let t99 = t98 * t39;
            let t100 = t41 * v_rho0;
            let t102 = f64x8::splat(1.0) / t43 / t100;
            let t104 = f64x8::splat(1.0) / t49;
            let t106 = t99 * v_sigma0 * t102 * t104;
            let t110 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t91 + t97 + t52 * t106 / f64x8::splat(60.0)));
            let t111 = t56 * t82;
            let t113 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t111)));
            let t116 = ((t60).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t113));
            let t117 = t116 * t30;
            let t118 = t117 * t76;
            let t119 = t6 * t118;
            let t121 = t64 * t93;
            let t122 = t121 * t76;
            let t123 = t6 * t122;
            let t124 = t123 / f64x8::splat(10.0);
            let t126 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t119 + t124));
            let tvrho0 = t54 + t80 + t7 * (t110 + t126);
            acc_vrho_0 = tvrho0;
            let t130 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t83)));
            let t133 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t130));
            let t134 = t133 * t30;
            let t135 = t134 * t50;
            let t136 = t6 * t135;
            let t139 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t136 + t97));
            let t141 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t111)));
            let t144 = ((t60).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t141));
            let t145 = t144 * t30;
            let t146 = t145 * t76;
            let t147 = t6 * t146;
            let t149 = t67 * v_rho1;
            let t151 = f64x8::splat(1.0) / t69 / t149;
            let t153 = f64x8::splat(1.0) / t75;
            let t155 = t99 * v_sigma2 * t151 * t153;
            let t159 = ((t55).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t147 + t124 + t78 * t155 / f64x8::splat(60.0)));
            let tvrho1 = t54 + t80 + t7 * (t139 + t159);
            acc_vrho_1 = tvrho1;
            let t164 = t98 * t39 * t45 * t104;
            let t167 = ((t1).select(f64x8::splat(0.0), -t52 * t164 / f64x8::splat(160.0)));
            let tvsigma0 = t7 * t167;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t170 = t98 * t39 * t71 * t153;
            let t173 = ((t55).select(f64x8::splat(0.0), -t78 * t170 / f64x8::splat(160.0)));
            let tvsigma2 = t7 * t173;
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
