//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1358/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1358(t12014: f64, t29860: f64, t29862: f64, t29865: f64, t29868: f64, t31525: f64, t31527: f64, t31533: f64, t31539: f64, t31542: f64, t31546: f64, t31551: f64, t31553: f64, t31556: f64, t4141: f64) -> f64 {
    let t38295 = t31525 + t31527 + t31533 + t31539 + 0.31616674039640166222e-2_f64 * t4141 * t12014 + t31542 + t31546 - t31551 + t31553 - t31556 + t29860 - t29862 - t29865 - t29868;
    t38295
}
