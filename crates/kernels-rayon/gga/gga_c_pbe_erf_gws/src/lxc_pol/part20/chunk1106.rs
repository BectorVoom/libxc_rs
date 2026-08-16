//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1106/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1106(t1176: f64, t2333: f64, t1180: f64, t13888: f64, t938: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64) {
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13895 = 119.0_f64 / 13824.0_f64 * t13894;
    let t13909 = t13888 * t938;
    let t13910 = t353 * t13909;
    let t13911 = t859 * t13910;
    (t13893, t13895, t13909, t13911)
}
