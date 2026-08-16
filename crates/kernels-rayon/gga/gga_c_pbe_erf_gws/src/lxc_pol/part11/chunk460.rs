//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 460/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk460(t1033: f64, t636: f64, t1045: f64, t582: f64, t211: f64, t1023: f64, t616: f64, t1018: f64, t185: f64, t1001: f64, t395: f64, t1014: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2747 = t1033 * t636;
    let t2749 = t582 * t1045;
    let t2750 = t211 * t2749;
    let t2753 = t582 * t1023;
    let t2754 = t616 * t2753;
    let t2756 = t582 * t1018;
    let t2757 = t185 * t2756;
    let t2760 = t395 * t1001;
    let t2773 = t401 * t1014;
    (t2747, t2749, t2750, t2753, t2754, t2756, t2757, t2760, t2773)
}
