//! LDA_XC_TIH vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 10 shared lines across all orders.
//! Delta: 10 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_XC_TIH vxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_xc_tih_vxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (10 lines) ---
        let t4 = f64::tanh(1.0953 + 0.0334789 * rho0 + 0.0334789 * rho1);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho0 + 0.152399 * rho1);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho0 + 0.0390837 * rho1);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho0 + 0.136598 * rho1);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho0 + 0.00496577 * rho1);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho0 + 4.02905 * rho1);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho0 + 0.0104352 * rho1);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho0 + 0.442455 * rho1);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
