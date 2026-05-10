//! GGA_X_RPBE fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 29 shared lines across all orders.
//! Delta: 21 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_rpbe_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_rpbe_kappa: f64,
    param_rpbe_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (29 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = param_rpbe_mu * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = 1.0 / param_rpbe_kappa;
        let t39 = f64::exp(-t21 * t25 * t29 * t33 * t34 / 24.0);
        let t42 = 1.0 + param_rpbe_kappa * (1.0 - t39);
        let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        // --- vxc delta (10 lines) ---
        let t52 = t30 * rho[ip];
        let t55 = t17 / t18 / t52;
        let t59 = t29 * t39;
        let t60 = t20 * t25 * t59;
        let t64 = piecewise3(t2, 0.0, -t6 * t17 / t31 * t42 / 8.0 + t6 * t55 * param_rpbe_mu * t60 / 24.0);
        let tvrho0 = 2.0 * rho[ip] * t64 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t72 = t25 * t28 * t39;
        let t73 = t21 * t72;
        let t76 = piecewise3(t2, 0.0, -t6 * t17 / t18 / t30 * t73 / 64.0);
        let tvsigma0 = 2.0 * rho[ip] * t76;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (21 lines) ---
        let t85 = t30 * t30;
        let t88 = t17 / t18 / t85;
        let t93 = t85 * t52;
        let t96 = param_rpbe_mu * param_rpbe_mu;
        let t98 = t6 * t17 / t93 * t96;
        let t99 = t20 * t20;
        let t102 = t99 / t23 / t22;
        let t103 = sigma[ip] * sigma[ip];
        let t106 = t27 * t34 * t39;
        let t107 = t102 * t103 * t106;
        let t111 = piecewise3(t2, 0.0, t6 * t17 / t31 / rho[ip] * t42 / 12.0 - t6 * t88 * param_rpbe_mu * t60 / 8.0 + t98 * t107 / 108.0);
        let tv2rho20 = 2.0 * rho[ip] * t111 + 4.0 * t64;
        v2rho2[ip] += tv2rho20;
        let t117 = t85 * t30;
        let t121 = t6 * t17 / t117 * t96;
        let t125 = t102 * t27 * sigma[ip] * t34 * t39;
        let t129 = piecewise3(t2, 0.0, 7.0 / 192.0 * t6 * t55 * t73 - t121 * t125 / 288.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t129 + 2.0 * t76;
        v2rhosigma[ip] += tv2rhosigma0;
        let t132 = t85 * rho[ip];
        let t137 = t102 * t106;
        let t140 = piecewise3(t2, 0.0, t6 * t17 / t132 * t96 * t137 / 768.0);
        let tv2sigma20 = 2.0 * rho[ip] * t140;
        v2sigma2[ip] += tv2sigma20;
    }
}
