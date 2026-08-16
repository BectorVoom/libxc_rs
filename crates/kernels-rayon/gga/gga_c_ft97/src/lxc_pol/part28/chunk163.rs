//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 163/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk163(t463: f64, t958: f64, t469: f64, t942: f64, t24: f64, t460: f64, t462: f64, t92: f64) -> (f64, f64, f64) {
    let t959 = t463 * t958;
    let t962 = t469 * t942;
    let t963 = t24 * t962;
    let t965 = -t460 - t462 * t959 / 3.0_f64 - t92 * t963;
    (t959, t963, t965)
}
