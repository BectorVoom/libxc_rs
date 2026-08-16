//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 798/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk798(t2105: f64, t339: f64, t6608: f64, t6609: f64, t860: f64, t2200: f64, t855: f64, t859: f64, t854: f64, t6104: f64, t823: f64, t850: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6610 = t2105 * t339;
    let t6612 = t6608 * t6609 * t6610;
    let t6614 = t6612 * t860 / 96.0_f64;
    let t6616 = t855 * t2200 * t859;
    let t6617 = t854 * t6616;
    let t6618 = 35.0_f64 / 144.0_f64 * t6617;
    let t6619 = t6104 * t823;
    let t6621 = t850 * t6619 * t852;
    (t6612, t6614, t6616, t6618, t6619, t6621)
}
