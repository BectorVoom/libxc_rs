//! GGA_X_PW86 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 72 shared lines across all orders.
//! Delta: 72 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_pw86_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (72 lines) ---
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
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_aa * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = t28 * t28;
        let t44 = param_bb * t43;
        let t46 = 1.0 / t31 / t30;
        let t47 = sigma0 * sigma0;
        let t48 = t46 * t47;
        let t49 = t35 * t35;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t36 / t50;
        let t56 = t30 * t30;
        let t58 = param_cc / t56;
        let t59 = t47 * sigma0;
        let t60 = t49 * t49;
        let t61 = 1.0 / t60;
        let t65 = 1.0 + t29 * t34 * t39 / 24.0 + t44 * t48 * t52 / 576.0 + t58 * t59 * t61 / 2304.0;
        let t66 = f64::powf(t65, 1.0 / 15.0);
        let t70 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t16;
        let t74 = piecewise5(t14, t11, t10, t15, t72 * t7);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t22, t77 * t75);
        let t80 = t79 * t26;
        let t81 = t33 * sigma2;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / t82;
        let t90 = sigma2 * sigma2;
        let t91 = t46 * t90;
        let t92 = t82 * t82;
        let t93 = t92 * rho1;
        let t95 = 1.0 / t83 / t93;
        let t99 = t90 * sigma2;
        let t100 = t92 * t92;
        let t101 = 1.0 / t100;
        let t105 = 1.0 + t29 * t81 * t86 / 24.0 + t44 * t91 * t95 / 576.0 + t58 * t99 * t101 / 2304.0;
        let t106 = f64::powf(t105, 1.0 / 15.0);
        let t110 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t106);
        let tzk0 = t70 + t110;
        zk[ip] += tzk0;
    }
}
