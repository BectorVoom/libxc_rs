//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 437/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk437(t18: f64, t464: f64, t463: f64, t458: f64, t963: f64, t1787: f64, t3009: f64, t2: f64, t942: f64, t1587: f64, t432: f64, t24: f64, t3103: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3140 = t464 * t18;
    let t3141 = t463 * t3140;
    let t3144 = t458 * t963;
    let t3146 = t1787 * t3009;
    let t3149 = t2 * t942;
    let t3151 = t1587 * t3149 * t432;
    let t3155 = t24 * t469 * t3103;
    (t3140, t3141, t3144, t3146, t3149, t3151, t3155)
}
