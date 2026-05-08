//! MGGA_X_RPPSCAN exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 83 shared lines across all orders.
//! Delta: 83 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_rppscan_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (83 lines) ---
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
        let t70 = t65 * t67 - t34 / 8.0;
        let t73 = param_eta * sigma[ip];
        let t74 = t28 * t33;
        let t77 = 3.0 / 10.0 * t40 * t24 + t73 * t74 / 8.0;
        let t78 = 1.0 / t77;
        let t79 = t70 * t78;
        let t80 = 1.0 - t79;
        let t82 = t80 * t80;
        let t84 = f64::exp(-t82 / 2.0);
        let t87 = 7.0 / 12960.0 * t62 * t34 + t60 * t80 * t84 / 100.0;
        let t88 = t87 * t87;
        let t89 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t88;
        let t94 = 1.0 + param_k1 * (1.0 - param_k1 / t89);
        let t95 = t79 <= 0.25e1;
        let t96 = 0.25e1 < t79;
        let t97 = piecewise3(t96, 0.25e1, t79);
        let t99 = t97 * t97;
        let t101 = t99 * t97;
        let t103 = t99 * t99;
        let t105 = t103 * t97;
        let t107 = t103 * t99;
        let t112 = piecewise3(t96, t79, 0.25e1);
        let t113 = 1.0 - t112;
        let t116 = f64::exp(param_c2 / t113);
        let t118 = piecewise3(t95, 1.0 - 0.667e0 * t97 - 0.4445555e0 * t99 - 0.663086601049e0 * t101 + 0.145129704449e1 * t103 - 0.887998041597e0 * t105 + 0.234528941479e0 * t107 - 0.23185843322e-1 * t103 * t101, -param_d * t116);
        let t119 = 1.0 - t118;
        let t122 = t94 * t119 + 0.1174e1 * t118;
        let t124 = f64::sqrt(3.0);
        let t125 = 1.0 / t23;
        let t126 = t40 * t125;
        let t127 = f64::sqrt(sigma[ip]);
        let t128 = t127 * t27;
        let t130 = 1.0 / t20 / rho[ip];
        let t132 = t126 * t128 * t130;
        let t133 = f64::sqrt(t132);
        let t137 = f64::exp(-0.98958e1 * t124 / t133);
        let t138 = 1.0 - t137;
        let t142 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t122 * t138);
        let tzk0 = 2.0 * t142;
        zk[ip] += tzk0;
    }
}
