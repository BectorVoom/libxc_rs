//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 949/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk949(t10702: f64, t617: f64, t1621: f64, t1620: f64, t10668: f64, t10670: f64, t10674: f64, t10678: f64, t10683: f64, t10687: f64, t10690: f64, t10695: f64, t10697: f64, t10699: f64, t10701: f64, t7526: f64, t7532: f64, t7541: f64, t7572: f64, t7573: f64) -> (f64, f64) {
    let t10703 = t10702 * t617;
    let t10704 = t1621 * t10703;
    let t10706 = 4.0_f64 / 15.0_f64 * t1620 * t10704;
    let t10707 = t10668 - t10670 + t7526 - t7532 + t10674 - t10678 + t10683 - 4.0_f64 / 27.0_f64 * t7541 - t10687 - t7572 + 0.66490888888888888886e-1_f64 * t7573 - t10690 - t10695 - t10697 + t10699 + t10701 - t10706;
    (t10706, t10707)
}
