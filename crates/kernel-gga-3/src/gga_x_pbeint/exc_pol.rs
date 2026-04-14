//! GGA_X_PBEINT exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 61 shared lines across all orders.
//! Delta: 61 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pbeint_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
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
        // --- shared preamble (61 lines) ---
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
        let t28 = param_muPBE - param_muGE;
        let t30 = M_CBRT6;
        let t31 = t28 * param_alpha * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t36 = t35 * sigma0;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / t37;
        let t42 = param_alpha * t30;
        let t43 = t36 * t41;
        let t46 = 1.0 + t42 * t43 / 24.0;
        let t47 = 1.0 / t46;
        let t53 = (param_muGE + t31 * t36 * t41 * t47 / 24.0) * t30;
        let t56 = param_kappa + t53 * t43 / 24.0;
        let t61 = 1.0 + param_kappa * (1.0 - param_kappa / t56);
        let t65 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t61);
        let t66 = rho1 <= dens_threshold;
        let t67 = -t16;
        let t69 = piecewise5(t14, t11, t10, t15, t67 * t7);
        let t70 = 1.0 + t69;
        let t71 = t70 <= zeta_threshold;
        let t72 = pow_1_3(t70);
        let t74 = piecewise3(t71, t22, t72 * t70);
        let t75 = t74 * t26;
        let t76 = t35 * sigma2;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = t76 * t81;
        let t85 = 1.0 + t42 * t82 / 24.0;
        let t86 = 1.0 / t85;
        let t92 = (param_muGE + t31 * t76 * t81 * t86 / 24.0) * t30;
        let t95 = param_kappa + t92 * t82 / 24.0;
        let t100 = 1.0 + param_kappa * (1.0 - param_kappa / t95);
        let t104 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t75 * t100);
        let tzk0 = t65 + t104;
        zk[ip] += tzk0;
    }
}
