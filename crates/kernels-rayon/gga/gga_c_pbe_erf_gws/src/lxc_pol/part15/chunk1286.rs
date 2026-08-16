//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1286/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1286(t53896: f64, t3950: f64, t833: f64, t850: f64, t9170: f64, t13944: f64, t2503: f64, t2409: f64, t28457: f64, t3965: f64, t14791: f64, t3066: f64, t51807: f64, t53874: f64, t53876: f64, t53878: f64, t53880: f64, t53884: f64, t53886: f64, t53889: f64, t53892: f64, t53894: f64, t8647: f64, t9283: f64) -> f64 {
    let t53897 = 7.0_f64 / 72.0_f64 * t53896;
    let t53904 = t850 * t9170 * t3950 * t833;
    let t53906 = t13944 * t2503;
    let t53910 = t3965 * t2409 * t28457;
    let t53912 = t53874 - t53876 / 256.0_f64 - t53878 / 24.0_f64 + t53880 / 16.0_f64 + t53884 / 96.0_f64 + 119.0_f64 / 6912.0_f64 * t53886 + t53889 / 96.0_f64 - t53892 / 48.0_f64 - t53894 / 96.0_f64 - t53897 - t3066 * t9283 * t14791 * t8647 / 8.0_f64 + t53904 / 96.0_f64 + t53906 / 96.0_f64 + 7.0_f64 / 4608.0_f64 * t51807 - t53910 / 96.0_f64;
    t53912
}
