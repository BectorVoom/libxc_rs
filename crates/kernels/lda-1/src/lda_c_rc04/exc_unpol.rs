//! LDA_C_RC04 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 18 shared lines across all orders.
//! Delta: 18 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_RC04 exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_rc04_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (18 lines) ---
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(1.0 <= zeta_threshold, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = M_CBRT3;
        let t9 = pow_1_3(1.0 / M_PI);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t18 = 4.88827 + 0.79425925 * t10 * t12 / t13;
        let t19 = f64::atan(t18);
        let t23 = t7 * t7;
        let t24 = t6 * (-0.655868 * t19 + 0.897889) * t23;
        let t26 = 1.0 / t9 * t11;
        let t28 = t24 * t26 * t13;
        let tzk0 = t28 / 3.0;
        zk[ip] += tzk0;
    }
}
