//! HYB_MGGA_X_M05 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_m05_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = t20 * param_csi_HF;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = sigma[ip] * t29;
        let t31 = rho[ip] * rho[ip];
        let t32 = t20 * t20;
        let t34 = 1.0 / t32 / t31;
        let t38 = 0.804 + 0.009146457198521547 * t27 * t30 * t34;
        let t41 = 1.804 - 0.646416 / t38;
        let t43 = param_a_1;
        let t44 = t22 * t22;
        let t46 = 3.0 / 10.0 * t44 * t25;
        let t47 = tau[ip] * t29;
        let t49 = 1.0 / t32 / rho[ip];
        let t50 = t47 * t49;
        let t51 = t46 - t50;
        let t52 = t43 * t51;
        let t53 = t46 + t50;
        let t54 = 1.0 / t53;
        let t56 = param_a_2;
        let t57 = t51 * t51;
        let t58 = t56 * t57;
        let t59 = t53 * t53;
        let t60 = 1.0 / t59;
        let t62 = param_a_3;
        let t63 = t57 * t51;
        let t64 = t62 * t63;
        let t65 = t59 * t53;
        let t66 = 1.0 / t65;
        let t68 = param_a_4;
        let t69 = t57 * t57;
        let t70 = t68 * t69;
        let t71 = t59 * t59;
        let t72 = 1.0 / t71;
        let t74 = param_a_5;
        let t75 = t69 * t51;
        let t76 = t74 * t75;
        let t77 = t71 * t53;
        let t78 = 1.0 / t77;
        let t80 = param_a_6;
        let t81 = t69 * t57;
        let t82 = t80 * t81;
        let t83 = t71 * t59;
        let t84 = 1.0 / t83;
        let t86 = param_a_7;
        let t87 = t69 * t63;
        let t88 = t86 * t87;
        let t89 = t71 * t65;
        let t90 = 1.0 / t89;
        let t92 = param_a_8;
        let t93 = t69 * t69;
        let t94 = t92 * t93;
        let t95 = t71 * t71;
        let t96 = 1.0 / t95;
        let t98 = param_a_9;
        let t99 = t93 * t51;
        let t100 = t98 * t99;
        let t102 = 1.0 / t95 / t53;
        let t104 = param_a_10;
        let t105 = t93 * t57;
        let t106 = t104 * t105;
        let t108 = 1.0 / t95 / t59;
        let t110 = param_a_11;
        let t112 = t110 * t93 * t63;
        let t114 = 1.0 / t95 / t65;
        let t116 = t100 * t102 + t106 * t108 + t112 * t114 + t52 * t54 + t58 * t60 + t64 * t66 + t70 * t72 + t76 * t78 + t82 * t84 + t88 * t90 + t94 * t96 + param_a_0;
        let t117 = t41 * t116;
        let t121 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t21 * t117);
        let tzk0 = 2.0 * t121;
        zk[ip] += tzk0;
    }
}
