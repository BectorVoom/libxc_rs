//! LDA_XC_ZLP exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 6 shared lines across all orders.
//! Delta: 6 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_XC_ZLP exc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_zlp_exc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (6 lines) ---
        let t1 = pow_1_3(rho[ip]);
        let t4 = 1.0 + 105.5562709925034 / t1;
        let t5 = f64::ln(t4);
        let t8 = 1.0 - 0.00947362 * t5 * t1;
        let t9 = t8 * t1;
        let tzk0 = -0.93222 * t9;
        zk[ip] += tzk0;
    }
}
