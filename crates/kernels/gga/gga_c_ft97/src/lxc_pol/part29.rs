//! GGA_C_FT97 lxc pol kernel — split part 29/36 (v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9_v4rho2sigma2_10).
//! Split sub-kernel: outputs [v4rho2sigma2, v4rho2sigma2, v4rho2sigma2, v4rho2sigma2].
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ft97.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part29_v4rho2sigma2_7_v4rho2sigma2_8_v4rho2sigma2_9_v4rho2sigma2_10(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v4rho2sigma2.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let tv4rho2sigma27 = 0.0;
        v4rho2sigma2[ip * 18 + 7] += tv4rho2sigma27;
        let tv4rho2sigma28 = 0.0;
        v4rho2sigma2[ip * 18 + 8] += tv4rho2sigma28;
        let tv4rho2sigma29 = 0.0;
        v4rho2sigma2[ip * 18 + 9] += tv4rho2sigma29;
        let tv4rho2sigma210 = 0.0;
        v4rho2sigma2[ip * 18 + 10] += tv4rho2sigma210;
    }
}
