//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 658/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk658(t2105: f64, t339: f64, t2200: f64, t855: f64, t859: f64, t899: f64, t912: f64, t923: f64) -> (f64, f64, f64) {
    let t6610 = t2105 * t339;
    let t6616 = t855 * t2200 * t859;
    let t6627 = t899 * t912 * t923;
    (t6610, t6616, t6627)
}
