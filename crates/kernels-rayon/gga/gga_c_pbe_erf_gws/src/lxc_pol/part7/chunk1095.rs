//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1095/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1095(t2100: f64, t353: f64, t859: f64, t898: f64, t938: f64, t2074: f64, t4386: f64, t11374: f64, t822: f64, t6161: f64, t376: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19599 = t859 * t353 * t898 * t2100 * t938;
    let t19602 = t898 * t2074;
    let t19605 = t4386 * t353 * t19602 * t938;
    let t19608 = t822 * t11374;
    let t19612 = t859 * t353 * t898 * t6161 * t938;
    let t19615 = t376 * t2100;
    let t19618 = t4386 * t353 * t19615 * t810;
    (t19599, t19602, t19605, t19608, t19612, t19615, t19618)
}
