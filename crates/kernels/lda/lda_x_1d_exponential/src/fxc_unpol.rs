//! LDA_X_1D_EXPONENTIAL fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_exponential.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_X_1D_EXPONENTIAL fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_exponential_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5::<f64>(t3, t5, t3, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t11 = t8 * M_PI * param_beta * rho[ip];
        let t12 = xc_integrate(func1, NULL, 1e-20, t11);
        let t14 = xc_integrate(func2, NULL, 1e-20, t11);
        let t16 = t14 / M_PI;
        let t17 = 1.0 / param_beta;
        let t18 = 1.0 / rho[ip];
        let t24 = piecewise3::<f64>(t4, 0.0, -0.07957747154594767 * (-t16 * t17 * t18 + t8 * t12) * t17);
        let tzk0 = 2.0 * t24;
        zk[ip] += tzk0;
        let t25 = param_beta * param_beta;
        let t26 = 1.0 / t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = 1.0 / t27;
        let t29 = t26 * t28;
        let t32 = piecewise3::<f64>(t4, 0.0, -0.07957747154594767 * t16 * t29);
        let tvrho0 = 2.0 * rho[ip] * t32 + 2.0 * t24;
        vrho[ip] += tvrho0;
        let t36 = t8 * t8;
        let t37 = t36 * M_PI;
        let t38 = M_PI * M_PI;
        let t42 = xc_E1_scaled(t36 * t38 * t25 * t27);
        let t47 = 1.0 / t27 / rho[ip];
        let t48 = t26 * t47;
        let t52 = piecewise3::<f64>(t4, 0.0, -0.07957747154594767 * t37 * t42 * t18 + 0.15915494309189535 * t16 * t48);
        let tv2rho20 = 2.0 * rho[ip] * t52 + 4.0 * t32;
        v2rho2[ip] += tv2rho20;
    }
}
