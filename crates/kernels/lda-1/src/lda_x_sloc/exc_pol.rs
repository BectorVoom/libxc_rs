//! LDA_X_SLOC exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 19 shared lines across all orders.
//! Delta: 19 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_SLOC exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_x_sloc_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (19 lines) ---
        let t1 = param_b + 1.0;
        let t3 = 1.0 / t1 / 2.0;
        let t4 = param_a * t3;
        let t5 = rho0 + rho1;
        let t6 = f64::powf(t5, param_b);
        let t7 = rho0 - rho1;
        let t8 = 1.0 / t5;
        let t9 = t7 * t8;
        let t10 = 1.0 + t9;
        let t11 = t10 <= zeta_threshold;
        let t12 = f64::powf(zeta_threshold, t1);
        let t13 = f64::powf(t10, t1);
        let t14 = piecewise3(t11, t12, t13);
        let t15 = 1.0 - t9;
        let t16 = t15 <= zeta_threshold;
        let t17 = f64::powf(t15, t1);
        let t18 = piecewise3(t16, t12, t17);
        let t19 = t14 + t18;
        let tzk0 = -t4 * t6 * t19;
        zk[ip] += tzk0;
    }
}
