//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 994/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk994(t10661: f64, t10668: f64, t10670: f64, t10674: f64, t10678: f64, t10683: f64, t10687: f64, t5929: f64, t5933: f64, t5938: f64, t5940: f64, t5944: f64, t7526: f64, t7532: f64, t8439: f64, t8440: f64) -> f64 {
    let t11211 = t10661 + t10668 - t10670 + t7526 - t7532 + t10674 - t10678 + t10683 - t10687 + t5929 + t5933 + 0.21642082724729686754e0_f64 * t5938 + 0.72140275749098955847e-1_f64 * t5940 - t5944 + t8439 + 16.0_f64 / 3.0_f64 * t8440;
    t11211
}
