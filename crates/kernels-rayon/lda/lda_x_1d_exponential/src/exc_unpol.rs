//! LDA_X_1D_EXPONENTIAL exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_exponential.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::integrate::{xc_integrate_lda_exponential_func1, xc_integrate_lda_exponential_func2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_1d_exponential_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t3, t5, t3, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t11 = t8 * M_PI * param_beta * rho[ip];
        let t12 = xc_integrate_lda_exponential_func1(t11);
        let t14 = xc_integrate_lda_exponential_func2(t11);
        let t16 = t14 / M_PI;
        let t17 = 1.0 / param_beta;
        let t18 = 1.0 / rho[ip];
        let t24 = piecewise3(t4, 0.0, -0.07957747154594767 * (-t16 * t17 * t18 + t8 * t12) * t17);
        let tzk0 = 2.0 * t24;
        zk[ip] += tzk0;
    }
}
