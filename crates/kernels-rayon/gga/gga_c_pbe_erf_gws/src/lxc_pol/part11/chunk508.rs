//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 508/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk508(t1758: f64, t3421: f64, t11: f64, t1764: f64, t3342: f64, t571: f64, t3346: f64, t572: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3422 = t1758 * t3421;
    let t3423 = t11 * t3422;
    let t3425 = t1764 * t3342;
    let t3426 = t571 * t3425;
    let t3427 = t11 * t3426;
    let t3429 = t572 * t3346;
    (t3422, t3423, t3425, t3426, t3427, t3429)
}
