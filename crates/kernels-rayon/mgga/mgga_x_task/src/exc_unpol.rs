//! MGGA_X_TASK exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_task.c`
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
pub fn mgga_x_task_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_task_c: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_h0x: f64,
    param_task_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_task_c = f64x8::splat(param_task_c);
    let param_task_bnu_0 = f64x8::splat(param_task_bnu_0);
    let param_task_bnu_1 = f64x8::splat(param_task_bnu_1);
    let param_task_bnu_2 = f64x8::splat(param_task_bnu_2);
    let param_task_bnu_3 = f64x8::splat(param_task_bnu_3);
    let param_task_bnu_4 = f64x8::splat(param_task_bnu_4);
    let param_task_anu_0 = f64x8::splat(param_task_anu_0);
    let param_task_anu_1 = f64x8::splat(param_task_anu_1);
    let param_task_anu_2 = f64x8::splat(param_task_anu_2);
    let param_task_h0x = f64x8::splat(param_task_h0x);
    let param_task_d = f64x8::splat(param_task_d);
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
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t26 = t21 / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t32 = t31 * t30;
            let t33 = f64x8::splat(1.0) / t32;
            let t36 = t26 * t29 * t33 / f64x8::splat(24.0);
            let t37 = (f64x8::splat(0.0)).simd_lt(t36);
            let t38 = ((t37).select(t36, f64x8::splat(0.0)));
            let t39 = ((t38).sqrt().sqrt());
            let t42 = (simd::exp(-param_task_c / t39));
            let t44 = ((t37).select(f64x8::splat(1.0) - t42, f64x8::splat(0.0)));
            let t46 = v_tau * v_tau;
            let t47 = t46 * t46;
            let t48 = t47 * t4;
            let t49 = param_task_bnu_0;
            let t50 = param_task_bnu_1;
            let t51 = param_task_bnu_2;
            let t52 = param_task_bnu_3;
            let t53 = param_task_bnu_4;
            let t54 = t49 + t50 + t51 + t52 + t53;
            let t55 = v_rho * v_tau;
            let t59 = f64x8::splat(1.0) / v_rho;
            let t61 = f64x8::splat(1.0) / v_tau;
            let t63 = (f64x8::splat(0.0)).simd_lt((f64x8::splat(0.9999999999) * t55 - f64x8::splat(0.125) * v_sigma) * t59 * t61);
            let t65 = f64x8::splat(8.0) * t55 - v_sigma;
            let t66 = t65 * t59;
            let t69 = ((t63).select(t66 * t61 / f64x8::splat(8.0), f64x8::splat(1e-10)));
            let t70 = t69 * t69;
            let t71 = t70 * t70;
            let t72 = t54 * t71;
            let t75 = t5 * f64x8::splat(M_PI);
            let t76 = t50 / f64x8::splat(2.0);
            let t77 = f64x8::splat(7.0) / f64x8::splat(2.0) * t52;
            let t78 = f64x8::splat(7.0) * t53;
            let t80 = t75 * (t49 + t76 - t51 - t77 - t78);
            let t81 = t31 * v_rho;
            let t82 = t46 * v_tau;
            let t83 = t81 * t82;
            let t84 = t70 * t69;
            let t88 = t30 * v_rho;
            let t89 = t19 * t88;
            let t90 = t5 * t5;
            let t91 = t90 * t22;
            let t92 = t89 * t91;
            let t93 = t4 * t4;
            let t94 = t92 * t93;
            let t97 = t49 - f64x8::splat(5.0) / f64x8::splat(3.0) * t51 + f64x8::splat(35.0) / f64x8::splat(3.0) * t53;
            let t98 = t97 * t46;
            let t99 = t98 * t70;
            let t102 = t30 * t30;
            let t103 = t102 * v_rho;
            let t104 = t22 * t22;
            let t105 = t103 * t104;
            let t106 = t49 - t76 - t51 + t77 - t78;
            let t107 = t105 * t106;
            let t108 = v_tau * t4;
            let t109 = t108 * t69;
            let t113 = t31 * t102 * t30;
            let t115 = t5 * t104 * f64x8::splat(M_PI);
            let t116 = t113 * t115;
            let t117 = t49 - t50 + t51 - t52 + t53;
            let t120 = f64x8::splat(108000.0) * t80 * t83 * t84 + f64x8::splat(29160.0) * t107 * t109 + f64x8::splat(6561.0) * t116 * t117 + f64x8::splat(30000.0) * t48 * t72 + f64x8::splat(48600.0) * t94 * t99;
            let t121 = t81 * t75;
            let t124 = f64x8::splat(9.0) * t121 + f64x8::splat(10.0) * t109;
            let t125 = t124 * t124;
            let t126 = t125 * t125;
            let t127 = f64x8::splat(1.0) / t126;
            let t129 = f64x8::splat(1.0) - t120 * t127;
            let t130 = param_task_anu_0;
            let t131 = param_task_anu_1;
            let t132 = param_task_anu_2;
            let t134 = t91 * (t130 - t131 + t132);
            let t138 = t4 * t75;
            let t140 = t130 - f64x8::splat(3.0) * t132;
            let t143 = f64x8::splat(24.0) * t138 * t140 * t32;
            let t145 = t130 + t131 + t132;
            let t146 = v_sigma * t93 * t145;
            let t149 = f64x8::splat(144.0) * t134 * t19 * t103 + (t143 + t146) * v_sigma;
            let t153 = f64x8::splat(12.0) * t75 * t32 + t4 * v_sigma;
            let t154 = t153 * t153;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t149 * t155 - param_task_h0x;
            let t158 = t129 * t157;
            let t159 = (simd::pow(t44, param_task_d));
            let t160 = t158 * t159;
            let t161 = param_task_h0x * t44 + t160;
            let t165 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t161));
            let tzk0 = f64x8::splat(2.0) * t165;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
