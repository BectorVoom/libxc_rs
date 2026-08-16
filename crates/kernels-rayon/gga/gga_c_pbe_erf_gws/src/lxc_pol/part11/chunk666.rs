//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 666/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk666(t267: f64, t7114: f64, t1791: f64, t641: f64, t1044: f64, t1018: f64, t1672: f64, t185: f64, t2789: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7117 = t7116 * t1044;
    let t7121 = t1672 * t1018;
    let t7122 = t185 * t7121;
    let t7130 = t2789 * t586;
    (t7115, t7116, t7117, t7121, t7122, t7130)
}
