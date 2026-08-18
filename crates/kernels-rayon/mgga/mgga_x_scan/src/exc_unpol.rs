//! MGGA_X_SCAN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_scan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
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
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t20 * t20;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t39 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t40 = t21 * t21;
        let t42 = t23 * t22;
        let t43 = 1.0 / t42;
        let t44 = t39 * t40 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t46 = t45 * t27;
        let t47 = t30 * t30;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t20 / t48;
        let t55 = f64::exp(-27.0 / 80.0 * t39 * t21 * t25 * t34);
        let t56 = t50 * t55;
        let t60 = f64::sqrt(146.0);
        let t61 = t60 * t21;
        let t62 = t61 * t25;
        let t65 = tau[ip] * t28;
        let t66 = t31 * rho[ip];
        let t67 = 1.0 / t66;
        let t73 = 5.0 / 9.0 * (t65 * t67 - t34 / 8.0) * t21 * t25;
        let t74 = 1.0 - t73;
        let t76 = t74 * t74;
        let t78 = f64::exp(-t76 / 2.0);
        let t81 = 7.0 / 12960.0 * t62 * t34 + t60 * t74 * t78 / 100.0;
        let t82 = t81 * t81;
        let t83 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t82;
        let t88 = 1.0 + param_k1 * (1.0 - param_k1 / t83);
        let t89 = t73 <= 1.0;
        let t90 = f64::ln(f64::EPSILON);
        let t93 = t90 / (-t90 + param_c1);
        let t94 = -t93 < t73;
        let t95 = t73 < -t93;
        let t96 = piecewise3(t95, t73, -t93);
        let t97 = param_c1 * t96;
        let t98 = 1.0 - t96;
        let t99 = 1.0 / t98;
        let t101 = f64::exp(-t97 * t99);
        let t102 = piecewise3(t94, 0.0, t101);
        let t103 = f64::abs(param_d);
        let t106 = f64::ln(f64::EPSILON / t103);
        let t109 = (-t106 + param_c2) / t106;
        let t110 = t73 < -t109;
        let t111 = piecewise3(t110, -t109, t73);
        let t112 = 1.0 - t111;
        let t115 = f64::exp(param_c2 / t112);
        let t117 = piecewise3(t110, 0.0, -param_d * t115);
        let t118 = piecewise3(t89, t102, t117);
        let t119 = 1.0 - t118;
        let t122 = t88 * t119 + 1.174 * t118;
        let t124 = f64::sqrt(3.0);
        let t125 = 1.0 / t23;
        let t126 = t40 * t125;
        let t127 = f64::sqrt(sigma[ip]);
        let t128 = t127 * t27;
        let t130 = 1.0 / t20 / rho[ip];
        let t132 = t126 * t128 * t130;
        let t133 = f64::sqrt(t132);
        let t137 = f64::exp(-9.8958 * t124 / t133);
        let t138 = 1.0 - t137;
        let t142 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t122 * t138);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
    }
}
