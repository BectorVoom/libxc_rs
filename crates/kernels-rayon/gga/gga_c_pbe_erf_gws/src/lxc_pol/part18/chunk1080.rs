//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1080/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1080(t11416: f64, t11418: f64, t11421: f64, t11448: f64, t11458: f64, t11463: f64, t11472: f64, t11477: f64, t11482: f64, t8823: f64, t8826: f64, t8835: f64) -> f64 {
    let t12149 = t11416 + t11418 + t11421 + t11448 + t8823 + t8826 + t8835 - t11458 + t11463 - t11472 + t11477 + t11482;
    t12149
}
