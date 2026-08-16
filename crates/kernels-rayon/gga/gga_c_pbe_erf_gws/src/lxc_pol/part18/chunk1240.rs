//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1240/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1240(t353: f64, t4183: f64, t814: f64, t859: f64, t20154: f64, t2376: f64, t4155: f64, t14724: f64, t343: f64, t361: f64, t14809: f64, t4414: f64) -> (f64, f64, f64, f64) {
    let t53464 = t859 * t353 * t4183 * t814;
    let t53472 = t20154 * t2376 * t4155 * t814;
    let t53496 = t361 * t14724 * t343;
    let t53503 = 7.0_f64 / 72.0_f64 * t4414 * t14809;
    (t53464, t53472, t53496, t53503)
}
