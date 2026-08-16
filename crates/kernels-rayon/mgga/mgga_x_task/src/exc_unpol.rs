//! MGGA_X_TASK exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_task.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_task_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_c: f64,
    param_task_d: f64,
    param_task_h0x: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = t11 + 1.0;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t26 = t21 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t32 = t31 * t30;
        let t33 = 1.0 / t32;
        let t36 = t26 * t29 * t33 / 24.0;
        let t37 = 0.0 < t36;
        let t38 = piecewise3(t37, t36, 0.0);
        let t39 = pow_1_4(t38);
        let t42 = f64::exp(-param_task_c / t39);
        let t44 = piecewise3(t37, 1.0 - t42, 0.0);
        let t46 = tau[ip] * tau[ip];
        let t47 = t46 * t46;
        let t48 = t47 * t4;
        let t49 = param_task_bnu_0;
        let t50 = param_task_bnu_1;
        let t51 = param_task_bnu_2;
        let t52 = param_task_bnu_3;
        let t53 = param_task_bnu_4;
        let t54 = t49 + t50 + t51 + t52 + t53;
        let t55 = rho[ip] * tau[ip];
        let t59 = 1.0 / rho[ip];
        let t61 = 1.0 / tau[ip];
        let t63 = 0.0 < (0.9999999999e0 * t55 - 0.125e0 * sigma[ip]) * t59 * t61;
        let t65 = 8.0 * t55 - sigma[ip];
        let t66 = t65 * t59;
        let t69 = piecewise3(t63, t66 * t61 / 8.0, 0.1e-9);
        let t70 = t69 * t69;
        let t71 = t70 * t70;
        let t72 = t54 * t71;
        let t75 = t5 * M_PI;
        let t76 = t50 / 2.0;
        let t77 = 7.0 / 2.0 * t52;
        let t78 = 7.0 * t53;
        let t80 = t75 * (t49 + t76 - t51 - t77 - t78);
        let t81 = t31 * rho[ip];
        let t82 = t46 * tau[ip];
        let t83 = t81 * t82;
        let t84 = t70 * t69;
        let t88 = t30 * rho[ip];
        let t89 = t19 * t88;
        let t90 = t5 * t5;
        let t91 = t90 * t22;
        let t92 = t89 * t91;
        let t93 = t4 * t4;
        let t94 = t92 * t93;
        let t97 = t49 - 5.0 / 3.0 * t51 + 35.0 / 3.0 * t53;
        let t98 = t97 * t46;
        let t99 = t98 * t70;
        let t102 = t30 * t30;
        let t103 = t102 * rho[ip];
        let t104 = t22 * t22;
        let t105 = t103 * t104;
        let t106 = t49 - t76 - t51 + t77 - t78;
        let t107 = t105 * t106;
        let t108 = tau[ip] * t4;
        let t109 = t108 * t69;
        let t113 = t31 * t102 * t30;
        let t115 = t5 * t104 * M_PI;
        let t116 = t113 * t115;
        let t117 = t49 - t50 + t51 - t52 + t53;
        let t120 = 108000.0 * t80 * t83 * t84 + 29160.0 * t107 * t109 + 6561.0 * t116 * t117 + 30000.0 * t48 * t72 + 48600.0 * t94 * t99;
        let t121 = t81 * t75;
        let t124 = 9.0 * t121 + 10.0 * t109;
        let t125 = t124 * t124;
        let t126 = t125 * t125;
        let t127 = 1.0 / t126;
        let t129 = 1.0 - t120 * t127;
        let t130 = param_task_anu_0;
        let t131 = param_task_anu_1;
        let t132 = param_task_anu_2;
        let t134 = t91 * (t130 - t131 + t132);
        let t138 = t4 * t75;
        let t140 = t130 - 3.0 * t132;
        let t143 = 24.0 * t138 * t140 * t32;
        let t145 = t130 + t131 + t132;
        let t146 = sigma[ip] * t93 * t145;
        let t149 = 144.0 * t134 * t19 * t103 + (t143 + t146) * sigma[ip];
        let t153 = 12.0 * t75 * t32 + t4 * sigma[ip];
        let t154 = t153 * t153;
        let t155 = 1.0 / t154;
        let t157 = t149 * t155 - param_task_h0x;
        let t158 = t129 * t157;
        let t159 = f64::powf(t44, param_task_d);
        let t160 = t158 * t159;
        let t161 = param_task_h0x * t44 + t160;
        let t165 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t161);
        let tzk0 = 2.0 * t165;
        zk[ip] += tzk0;
    }
}
