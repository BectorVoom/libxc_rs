//! MGGA_X_FT98 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_ft98_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = param_a1 * sigma[ip];
        let t22 = M_CBRT2;
        let t23 = t22 * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = t19 * t19;
        let t27 = 1.0 / t25 / t24;
        let t28 = t23 * t27;
        let t30 = t21 * t28 + 1.0;
        let t31 = f64::sqrt(t30);
        let t32 = param_a * t31;
        let t33 = param_b1 * sigma[ip];
        let t35 = t28 * t33 + 1.0;
        let t36 = pow_1_4(t35);
        let t37 = t36 * t36;
        let t38 = t37 * t36;
        let t39 = 1.0 / t38;
        let t40 = t32 * t39;
        let t41 = sigma[ip] * t23;
        let t42 = t41 * t27;
        let t44 = lapl[ip] * t23;
        let t46 = 1.0 / t25 / rho[ip];
        let t48 = -t44 * t46 + t42;
        let t49 = t48 * t48;
        let t50 = param_a2 * t49;
        let t51 = 1.0 + t42;
        let t52 = t51 * t51;
        let t53 = 1.0 / t52;
        let t56 = param_b * (t50 * t53 + 1.0);
        let t57 = param_b2 * param_b2;
        let t59 = f64::sqrt(t57 + 1.0);
        let t60 = t59 - param_b2;
        let t61 = sigma[ip] * sigma[ip];
        let t62 = t61 * t22;
        let t63 = t24 * t24;
        let t64 = t63 * rho[ip];
        let t66 = 1.0 / t19 / t64;
        let t67 = t62 * t66;
        let t68 = 2.0 * t67;
        let t69 = lapl[ip] * lapl[ip];
        let t70 = t69 * t22;
        let t71 = t24 * rho[ip];
        let t73 = 1.0 / t19 / t71;
        let t74 = t70 * t73;
        let t75 = 2.0 * t74;
        let t76 = t68 - t75 - param_b2;
        let t77 = pow_1_4(f64::EPSILON);
        let t78 = 1.0 / t77;
        let t79 = t76 < -t78;
        let t85 = t76 * t76;
        let t86 = t85 * t76;
        let t87 = 1.0 / t86;
        let t89 = t85 * t85;
        let t90 = t89 * t76;
        let t91 = 1.0 / t90;
        let t96 = piecewise3(0.0 < t76, t76, -t76);
        let t97 = t96 < t77;
        let t100 = t89 * t85;
        let t102 = t89 * t89;
        let t105 = -t78 < t76;
        let t106 = piecewise3(t105, t76, -t78);
        let t107 = t106 * t106;
        let t108 = 1.0 + t107;
        let t109 = f64::sqrt(t108);
        let t110 = t106 + t109;
        let t112 = piecewise5(t79, -4.0 * t67 + 4.0 * t74 + 2.0 * param_b2 - 1.0 / t76 / 2.0 + t87 / 8.0 - t91 / 16.0, t97, 1.0 - t68 + t75 + param_b2 + t85 / 2.0 - t89 / 8.0 + t100 / 16.0 - 5.0 / 128.0 * t102, 1.0 / t110);
        let t114 = t112 * t60 + 1.0;
        let t115 = t22 - 1.0;
        let t116 = t115 * t60;
        let t118 = t112 * t116 + 1.0;
        let t119 = t118 * t118;
        let t120 = t119 * t118;
        let t121 = 1.0 / t120;
        let t122 = t114 * t121;
        let t123 = t122 * t49;
        let t125 = t123 * t56 + t40 * t42 + 1.0;
        let t126 = t4 * t4;
        let t127 = 1.0 / M_PI;
        let t128 = pow_1_3(t127);
        let t129 = t128 * t128;
        let t130 = t126 * t129;
        let t131 = M_CBRT4;
        let t133 = param_b * sigma[ip];
        let t137 = 1.0 + 81.0 / 4.0 * t130 * t131 * t133 * t28;
        let t138 = 1.0 / t137;
        let t139 = t125 * t138;
        let t140 = f64::sqrt(t139);
        let t144 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t140);
        let tzk0 = 2.0 * t144;
        zk[ip] += tzk0;
    }
}
