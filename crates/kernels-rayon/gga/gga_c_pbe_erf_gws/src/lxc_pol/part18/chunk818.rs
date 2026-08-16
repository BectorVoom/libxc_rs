//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 818/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk818(t2659: f64, t586: f64, t2816: f64, t636: f64, t197: f64, t589: f64, t172: f64, t2824: f64, t184: f64, t2684: f64, t5137: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t7136 = t2659 * t586;
    let t7147 = 8.0_f64 / 45.0_f64 * t2816 * t636;
    let t7148 = t589 * t197;
    let t7170 = t172 * t2824;
    let t7171 = t7170 * t184;
    let t7188 = t5137 * t2684;
    let t7190 = 16.0_f64 / 135.0_f64 * t639 * t7188;
    (t7136, t7147, t7148, t7171, t7190)
}
