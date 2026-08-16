//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 865/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk865(t38279: f64, t38280: f64, t8653: f64, t8657: f64, t8660: f64, t9381: f64, t9934: f64, t9937: f64, t9940: f64, t9946: f64, t9950: f64, t38292: f64, t8673: f64, t8681: f64, t8683: f64, t9412: f64, t9977: f64, t9979: f64, t9981: f64, t9983: f64, t9987: f64, t9992: f64) -> (f64, f64) {
    let t44554 = -t38279 - t38280 - 0.25538759935978703639e-4_f64 * t8653 - t9381 + t9934 + t9937 + t9940 + t9946 - 0.36366215538993788972e-1_f64 * t8657 + 0.20455996240684006297e-1_f64 * t8660 + t9950;
    let t44563 = t9977 - t9979 - t9981 - t9983 - t9987 + t9992 + 0.14546486215597515588e0_f64 * t8673 - t9412 - t38292 + 0.25538759935978703639e-4_f64 * t8681 - 0.25538759935978703639e-4_f64 * t8683;
    (t44554, t44563)
}
