//! GGA_X_Q2D exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q2d_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 0.804e0 + 5.0 / 972.0 * t40;
        let t45 = 0.1804e1 - 0.646416e0 / t42;
        let t46 = t28 * t28;
        let t48 = 1.0 / t30 / t29;
        let t49 = t46 * t48;
        let t50 = sigma0 * sigma0;
        let t51 = t34 * t34;
        let t52 = t51 * rho0;
        let t54 = 1.0 / t35 / t52;
        let t58 = 100.0 - t49 * t50 * t54 / 576.0;
        let t60 = 1.0 / t30;
        let t61 = t46 * t60;
        let t62 = f64::sqrt(sigma0);
        let t64 = 1.0 / t35 / rho0;
        let t66 = t61 * t62 * t64;
        let t67 = f64::powf(t66, 0.35e1);
        let t69 = 1.0 + t40 / 24.0;
        let t72 = t45 * t58 + 0.87153829697982569831e-4 * t67 * t69;
        let t73 = t27 * t72;
        let t74 = t29 * t29;
        let t75 = 1.0 / t74;
        let t76 = t50 * sigma0;
        let t78 = t51 * t51;
        let t79 = 1.0 / t78;
        let t82 = 100.0 + t75 * t76 * t79 / 2304.0;
        let t83 = 1.0 / t82;
        let t84 = t73 * t83;
        let t87 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t84);
        let t88 = rho1 <= dens_threshold;
        let t89 = -t16;
        let t91 = piecewise5(t14, t11, t10, t15, t89 * t7);
        let t92 = 1.0 + t91;
        let t93 = t92 <= zeta_threshold;
        let t94 = pow_1_3(t92);
        let t96 = piecewise3(t93, t22, t94 * t92);
        let t97 = t5 * t96;
        let t98 = rho1 * rho1;
        let t99 = pow_1_3(rho1);
        let t100 = t99 * t99;
        let t102 = 1.0 / t100 / t98;
        let t104 = t33 * sigma2 * t102;
        let t106 = 0.804e0 + 5.0 / 972.0 * t104;
        let t109 = 0.1804e1 - 0.646416e0 / t106;
        let t110 = sigma2 * sigma2;
        let t111 = t98 * t98;
        let t112 = t111 * rho1;
        let t114 = 1.0 / t99 / t112;
        let t118 = 100.0 - t49 * t110 * t114 / 576.0;
        let t120 = f64::sqrt(sigma2);
        let t122 = 1.0 / t99 / rho1;
        let t124 = t61 * t120 * t122;
        let t125 = f64::powf(t124, 0.35e1);
        let t127 = 1.0 + t104 / 24.0;
        let t130 = t109 * t118 + 0.87153829697982569831e-4 * t125 * t127;
        let t131 = t27 * t130;
        let t132 = t110 * sigma2;
        let t134 = t111 * t111;
        let t135 = 1.0 / t134;
        let t138 = 100.0 + t75 * t132 * t135 / 2304.0;
        let t139 = 1.0 / t138;
        let t140 = t131 * t139;
        let t143 = piecewise3(t88, 0.0, -3.0 / 8.0 * t97 * t140);
        let tzk0 = t87 + t143;
        zk[ip] += tzk0;
    }
}
