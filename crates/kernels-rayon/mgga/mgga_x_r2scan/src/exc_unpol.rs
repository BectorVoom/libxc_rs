//! MGGA_X_R2SCAN exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_r2scan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t22 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t25;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = v_sigma * v_sigma;
            let t32 = f64x8::splat(M_CBRT2);
            let t33 = v_rho * v_rho;
            let t34 = t33 * t33;
            let t35 = t34 * v_rho;
            let t37 = f64x8::splat(1.0) / t20 / t35;
            let t38 = t32 * t37;
            let t39 = param_dp2 * param_dp2;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t45 = (simd::exp(-t29 * t30 * t38 * t41 / f64x8::splat(288.0)));
            let t49 = (-f64x8::splat(0.162742215233874) * t22 * t45 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t23;
            let t50 = t26 * t26;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t49 * t51;
            let t53 = t32 * t32;
            let t54 = v_sigma * t53;
            let t55 = t20 * t20;
            let t57 = f64x8::splat(1.0) / t55 / t33;
            let t58 = t54 * t57;
            let t61 = param_k1 + t52 * t58 / f64x8::splat(24.0);
            let t65 = param_k1 * (f64x8::splat(1.0) - param_k1 / t61);
            let t66 = v_tau * t53;
            let t67 = t55 * v_rho;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = t66 * t68 - t58 / f64x8::splat(8.0);
            let t75 = t53 * t57;
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t24 * t50 + param_eta * v_sigma * t75 / f64x8::splat(8.0);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t71 * t79;
            let t81 = (t80).simd_le(f64x8::splat(0.0));
            let t82 = (f64x8::splat(0.0)).simd_lt(t80);
            let t83 = ((t82).select(f64x8::splat(0.0), t80));
            let t84 = param_c1 * t83;
            let t85 = f64x8::splat(1.0) - t83;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = (simd::exp(-t84 * t86));
            let t89 = (t80).simd_le(f64x8::splat(2.5));
            let t90 = (f64x8::splat(2.5)).simd_lt(t80);
            let t91 = ((t90).select(f64x8::splat(2.5), t80));
            let t93 = t91 * t91;
            let t95 = t93 * t91;
            let t97 = t93 * t93;
            let t99 = t97 * t91;
            let t101 = t97 * t93;
            let t106 = ((t90).select(t80, f64x8::splat(2.5)));
            let t107 = f64x8::splat(1.0) - t106;
            let t110 = (simd::exp(param_c2 / t107));
            let t112 = ((t81).select(t88, (t89).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t91 - f64x8::splat(0.4445555) * t93 - f64x8::splat(0.663086601049) * t95 + f64x8::splat(1.45129704449) * t97 - f64x8::splat(0.887998041597) * t99 + f64x8::splat(0.234528941479) * t101 - f64x8::splat(0.023185843322) * t97 * t95, -param_d * t110)));
            let t113 = f64x8::splat(0.174) - t65;
            let t115 = t112 * t113 + t65 + f64x8::splat(1.0);
            let t117 = ((f64x8::splat(3.0)).sqrt());
            let t118 = f64x8::splat(1.0) / t26;
            let t119 = t24 * t118;
            let t120 = ((v_sigma).sqrt());
            let t121 = t120 * t32;
            let t123 = f64x8::splat(1.0) / t20 / v_rho;
            let t125 = t119 * t121 * t123;
            let t126 = ((t125).sqrt());
            let t130 = (simd::exp(-f64x8::splat(9.8958) * t117 / t126));
            let t131 = f64x8::splat(1.0) - t130;
            let t135 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t115 * t131));
            let tzk0 = f64x8::splat(2.0) * t135;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
