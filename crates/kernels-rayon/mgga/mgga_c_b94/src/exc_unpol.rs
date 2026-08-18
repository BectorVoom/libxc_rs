//! MGGA_C_B94 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRTPI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b94_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_cab: f64,
    param_css: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] * param_cab;
        let t4 = rho[ip] / 2.0 <= dens_threshold;
        let t5 = M_CBRT2;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t11 = t10 * rho[ip];
        let t12 = pow_1_3(t11);
        let t13 = 1.0 / t12;
        let t14 = t5 * t13;
        let t15 = M_CBRTPI;
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = t5 * t5;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t19 * t19;
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t25 = tau[ip] * param_gamma;
        let t28 = param_gamma * sigma[ip];
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t20 / t29;
        let t35 = f64::abs(lapl[ip] * t22 / 2.0 - 2.0 * t25 * t22 + t28 * t31 / 4.0);
        let t38 = t18 * t35 / 3.0 < 5e-13;
        let t39 = lapl[ip] * t18;
        let t42 = t18 * t22;
        let t45 = t18 * t31;
        let t48 = t39 * t22 / 6.0 - 2.0 / 3.0 * t25 * t42 + t28 * t45 / 12.0;
        let t49 = 0.0 < t48;
        let t50 = piecewise3(t49, 5e-13, -5e-13);
        let t51 = piecewise3(t38, t50, t48);
        let t52 = xc_mgga_x_br89_get_x(t51);
        let t54 = f64::exp(t52 / 3.0);
        let t55 = 1.0 / t54;
        let t56 = f64::exp(-t52);
        let t58 = 1.0 + t52 / 2.0;
        let t59 = t56 * t58;
        let t60 = 1.0 - t59;
        let t61 = 1.0 / t60;
        let t62 = t55 * t61;
        let t63 = t62 * t52;
        let t66 = piecewise3(t4, 0.0, t17 * t63 / 2.0);
        let t67 = param_cab * t66;
        let t68 = 2.0 * t67;
        let t69 = 1.0 + t68;
        let t70 = f64::ln(t69);
        let t71 = t68 - t70;
        let t74 = 0.4 * t2 * t66 * t71;
        let t75 = t10 * t10;
        let t76 = pow_1_3(t10);
        let t77 = t76 * t76;
        let t78 = t77 * t75;
        let t79 = t78 * t18;
        let t80 = tau[ip] * t18;
        let t83 = sigma[ip] * t18;
        let t86 = 2.0 * t80 * t22 - t83 * t31 / 4.0;
        let t87 = t21 * t86;
        let t88 = param_css * param_css;
        let t89 = t88 * t88;
        let t90 = t87 * t89;
        let t91 = t79 * t90;
        let t93 = 1.0 / t12 / t11;
        let t94 = t54 * t54;
        let t95 = t94 * t94;
        let t96 = 1.0 / t95;
        let t97 = t93 * t96;
        let t98 = t60 * t60;
        let t99 = t98 * t98;
        let t100 = 1.0 / t99;
        let t101 = t52 * t52;
        let t102 = t101 * t101;
        let t103 = t100 * t102;
        let t104 = param_css * t5;
        let t105 = t104 * t13;
        let t106 = t16 * t55;
        let t107 = t61 * t52;
        let t111 = 1.0 + t105 * t106 * t107 / 2.0;
        let t112 = f64::ln(t111);
        let t113 = 1.0 / param_css;
        let t114 = t112 * t113;
        let t115 = t18 * t12;
        let t116 = t114 * t115;
        let t117 = t15 * t54;
        let t118 = 1.0 / t52;
        let t119 = t60 * t118;
        let t122 = -t116 * t117 * t119 + 1.0;
        let t123 = t103 * t122;
        let t124 = t97 * t123;
        let t127 = piecewise3(t4, 0.0, -0.0005433422936572482 * t91 * t124);
        let t128 = 2.0 * t127;
        let tzk0 = -t74 + t128;
        zk[ip] += tzk0;
    }
}
