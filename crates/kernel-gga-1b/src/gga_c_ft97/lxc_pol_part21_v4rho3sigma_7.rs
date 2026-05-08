//! GGA_C_FT97 lxc pol kernel — split part 21/34 (v4rho3sigma_7).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ft97.c`.
//! Split sub-kernel: outputs [v4rho3sigma] (27 lines).
//! Each sub-kernel recomputes its dependency chain from inputs.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_7(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v4rho3sigma.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let tv4rho3sigma7 = 0.0;
        v4rho3sigma[ip * 12 + 7] += tv4rho3sigma7;
    }
}
