//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1246/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1246(t13972: f64, t14608: f64, t898: f64, t911: f64, t3973: f64, t13953: f64, t14787: f64, t14781: f64, t14001: f64, t3062: f64, t14772: f64, t14466: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54491 = t13972 * t14608;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54504 = t13953 * t14787;
    let t54531 = t13953 * t14781;
    let t54535 = t14001 * t3062;
    let t54537 = t14001 * t14772;
    let t54566 = t14001 * t14466;
    (t54491, t54499, t54504, t54531, t54535, t54537, t54566)
}
