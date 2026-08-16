//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 964/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk964(t1078: f64, t11764: f64, t3427: f64, t3757: f64, t277: f64, t641: f64, t11755: f64, t11522: f64, t7073: f64, t9799: f64, t7451: f64, t9396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11765 = t11764 * t1078;
    let t11767 = t3757 * t3427;
    let t11769 = t277 * t641;
    let t11770 = t11769 * t11755;
    let t11772 = t7073 * t11522;
    let t11773 = t11772 * t9799;
    let t11775 = t7451 * t11522;
    let t11776 = t11775 * t9396;
    (t11765, t11767, t11769, t11770, t11772, t11773, t11775, t11776)
}
