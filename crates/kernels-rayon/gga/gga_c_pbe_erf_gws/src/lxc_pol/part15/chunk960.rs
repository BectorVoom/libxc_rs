//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 960/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk960(t7923: f64, t7927: f64, t7931: f64, t7934: f64, t7939: f64, t7943: f64, t7944: f64, t7947: f64, t7949: f64, t7953: f64, t7955: f64, t7958: f64, t7961: f64, t7965: f64, t7968: f64, t7970: f64, t7971: f64) -> f64 {
    let t8456 = -t7923 + t7927 - t7931 + t7934 - t7939 - t7943 - t7944 - t7947 - t7949 + t7953 + t7955 - t7958 - t7961 + t7965 + t7968 + t7970 + t7971;
    t8456
}
