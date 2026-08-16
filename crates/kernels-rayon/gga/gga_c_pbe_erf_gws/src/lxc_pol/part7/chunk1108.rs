//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1108/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1108(t6824: f64, t9270: f64, t328: f64, t6045: f64, t824: f64, t822: f64, t833: f64, t2397: f64, t6745: f64, t2242: f64, t2355: f64, t6810: f64, t8801: f64) -> (f64, f64, f64, f64, f64) {
    let t19836 = t9270 * t6824;
    let t19839 = t824 * t328 * t6045;
    let t19841 = t822 * t19839 * t833;
    let t19843 = t6745 * t2397;
    let t19845 = t2242 * t2355;
    let t19857 = t8801 * t6810;
    (t19836, t19841, t19843, t19845, t19857)
}
