//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2832/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832(t11263: f64, t3169: f64, t3043: f64, t3140: f64, t3149: f64, t3160: f64, t11874: f64, t16048: f64, t12046: f64, t15905: f64, t994: f64, t3114: f64, t42416: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42656 = t3169 * t11263;
    let t42664 = t3043 * t3140;
    let t42665 = t42664 * t3149;
    let t42672 = t42664 * t3160;
    let t42675 = t11874 * t16048;
    let t42690 = t994 * t12046 * t15905;
    let t42695 = t3114 * t42416;
    (t42656, t42665, t42672, t42675, t42690, t42695)
}
