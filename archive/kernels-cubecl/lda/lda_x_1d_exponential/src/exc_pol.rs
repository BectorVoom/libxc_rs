//! LDA_X_1D_EXPONENTIAL exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_exponential.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::integrate::{xc_integrate_lda_exponential_func1, xc_integrate_lda_exponential_func2};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};

/// LDA_X_1D_EXPONENTIAL exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_exponential_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = zeta_threshold - 1.0;
        let t11 = 1.0 - t5 <= zeta_threshold;
        let t12 = -t9;
        let t13 = piecewise5::<f64>(t7, t9, t11, t12, t5);
        let t14 = 1.0 + t13;
        let t15 = t14 * M_PI;
        let t16 = param_beta * t3;
        let t17 = t15 * t16;
        let t18 = xc_integrate_lda_exponential_func1::<f64>(t17);
        let t20 = xc_integrate_lda_exponential_func2::<f64>(t17);
        let t21 = 1.0 / M_PI;
        let t22 = t20 * t21;
        let t23 = 1.0 / param_beta;
        let t24 = t23 * t4;
        let t29 = piecewise3::<f64>(t8, 0.0, -0.07957747154594767 * (t14 * t18 - t22 * t24) * t23);
        let t31 = rho1 <= dens_threshold || t11;
        let t32 = piecewise5::<f64>(t11, t9, t7, t12, -t5);
        let t33 = 1.0 + t32;
        let t34 = t33 * M_PI;
        let t35 = t34 * t16;
        let t36 = xc_integrate_lda_exponential_func1::<f64>(t35);
        let t38 = xc_integrate_lda_exponential_func2::<f64>(t35);
        let t39 = t38 * t21;
        let t44 = piecewise3::<f64>(t31, 0.0, -0.07957747154594767 * (-t39 * t24 + t33 * t36) * t23);
        let tzk0 = t29 + t44;
        zk[ip] += tzk0;
    }
}
