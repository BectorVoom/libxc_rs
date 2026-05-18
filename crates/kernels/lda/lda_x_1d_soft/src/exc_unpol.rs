//! LDA_X_1D_SOFT exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_soft.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_X_1D_SOFT exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_1d_soft_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t12 = xc_integrate(func1, NULL, 0.0, t11);
        let t14 = xc_integrate(func2, NULL, 0.0, t11);
        let t15 = 1.0 / M_PI;
        let t16 = t14 * t15;
        let t17 = 1.0 / param_beta;
        let t18 = 1.0 / rho[ip];
        let t19 = t17 * t18;
        let t24 = piecewise3::<f64>(t4, 0.0, -0.07957747154594767 * (t8 * t12 - t16 * t19) * t17);
        let tzk0 = 2.0 * t24;
        zk[ip] += tzk0;
    }
}
