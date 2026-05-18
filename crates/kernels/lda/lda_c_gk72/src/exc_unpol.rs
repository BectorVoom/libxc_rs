//! LDA_C_GK72 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gk72.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 exc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5::<f64>(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        zk[ip] += tzk0;
    }
}
