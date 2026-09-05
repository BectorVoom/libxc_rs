//! MGGA_X_MVS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvs_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c1 = f64x8::splat(param_c1);
    let param_e1 = f64x8::splat(param_e1);
    let param_k0 = f64x8::splat(param_k0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t7 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_tau * t22;
            let t24 = t20 * t20;
            let t26 = f64x8::splat(1.0) / t24 / v_rho;
            let t28 = v_sigma * t22;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t24 / t29;
            let t34 = t23 * t26 - t28 * t31 / f64x8::splat(8.0);
            let t35 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t44 = param_k0 * (f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t34 * t35 * t40);
            let t45 = t34 * t34;
            let t47 = t35 * t35;
            let t49 = f64x8::splat(1.0) / t38 / t37;
            let t50 = t47 * t49;
            let t53 = f64x8::splat(1.0) + f64x8::splat(25.0) / f64x8::splat(81.0) * param_e1 * t45 * t50;
            let t54 = t53 * t53;
            let t55 = t45 * t45;
            let t57 = t37 * t37;
            let t59 = f64x8::splat(1.0) / t39 / t57;
            let t60 = t35 * t59;
            let t63 = t54 + f64x8::splat(1250.0) / f64x8::splat(2187.0) * param_c1 * t55 * t60;
            let t64 = ((t63).sqrt().sqrt());
            let t65 = f64x8::splat(1.0) / t64;
            let t67 = t44 * t65 + f64x8::splat(1.0);
            let t71 = v_sigma * v_sigma;
            let t73 = t29 * t29;
            let t74 = t73 * v_rho;
            let t76 = f64x8::splat(1.0) / t20 / t74;
            let t80 = f64x8::splat(1.0) + param_b * t47 * t49 * t71 * t21 * t76 / f64x8::splat(288.0);
            let t81 = (simd::pow(t80, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t82 = f64x8::splat(1.0) / t81;
            let t86 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t67 * t82));
            let tzk0 = f64x8::splat(2.0) * t86;
            acc_zk = tzk0;
            let t87 = f64x8::splat(1.0) / t24;
            let t94 = t29 * v_rho;
            let t96 = f64x8::splat(1.0) / t24 / t94;
            let t99 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t23 * t31 + t28 * t96 / f64x8::splat(3.0);
            let t100 = param_k0 * t99;
            let t101 = t35 * t40;
            let t102 = t101 * t65;
            let t106 = f64x8::splat(1.0) / t64 / t63;
            let t107 = t53 * param_e1;
            let t108 = t107 * t34;
            let t113 = param_c1 * t45 * t34;
            let t117 = f64x8::splat(100.0) / f64x8::splat(81.0) * t108 * t50 * t99 + f64x8::splat(5000.0) / f64x8::splat(2187.0) * t113 * t60 * t99;
            let t118 = t106 * t117;
            let t121 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t100 * t102 - t44 * t118 / f64x8::splat(4.0);
            let t126 = t73 * t29;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t18 * t127;
            let t130 = t7 * t128 * t67;
            let t133 = f64x8::splat(1.0) / t81 / t80 * param_b;
            let t134 = t133 * t47;
            let t137 = t134 * t49 * t71 * t21;
            let t141 = ((t3).select(f64x8::splat(0.0), -t19 * t87 * t67 * t82 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t121 * t82 - t130 * t137 / f64x8::splat(1152.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t141 + f64x8::splat(2.0) * t86;
            acc_vrho = tvrho0;
            let t144 = param_k0 * t22;
            let t145 = t144 * t31;
            let t146 = t145 * t102;
            let t148 = t22 * t31;
            let t149 = t50 * t148;
            let t150 = t108 * t149;
            let t152 = t113 * t35;
            let t153 = t59 * t22;
            let t154 = t153 * t31;
            let t155 = t152 * t154;
            let t157 = -f64x8::splat(25.0) / f64x8::splat(162.0) * t150 - f64x8::splat(625.0) / f64x8::splat(2187.0) * t155;
            let t158 = t106 * t157;
            let t161 = f64x8::splat(5.0) / f64x8::splat(72.0) * t146 - t44 * t158 / f64x8::splat(4.0);
            let t166 = f64x8::splat(1.0) / t74;
            let t167 = t18 * t166;
            let t169 = t7 * t167 * t67;
            let t172 = t134 * t49 * v_sigma * t21;
            let t176 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t161 * t82 + t169 * t172 / f64x8::splat(3072.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t176;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t178 = t144 * t26;
            let t181 = t22 * t26;
            let t182 = t50 * t181;
            let t185 = t153 * t26;
            let t188 = f64x8::splat(100.0) / f64x8::splat(81.0) * t108 * t182 + f64x8::splat(5000.0) / f64x8::splat(2187.0) * t152 * t185;
            let t189 = t106 * t188;
            let t192 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t178 * t102 - t44 * t189 / f64x8::splat(4.0);
            let t197 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t192 * t82));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t197;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
