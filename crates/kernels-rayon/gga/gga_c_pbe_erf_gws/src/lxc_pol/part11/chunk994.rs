//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 994/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk994(t3788: f64, t6616: f64, t3810: f64, t6717: f64, t19561: f64, t3802: f64, t11807: f64, t6331: f64, t27222: f64, t3123: f64, t3861: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36699 = t3788 * t6616;
    let t36803 = t6717 * t3810;
    let t36814 = t3802 * t19561;
    let t36837 = t6331 * t11807;
    let t36869 = t3123 * t27222;
    let t36880 = t904 * t3861;
    (t36699, t36803, t36814, t36837, t36869, t36880)
}
