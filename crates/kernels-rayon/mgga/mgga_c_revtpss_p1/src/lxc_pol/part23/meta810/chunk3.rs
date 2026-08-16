//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2655/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655(t2439: f64, t6132: f64, t6135: f64, t19013: f64, t698: f64, t19016: f64, t6138: f64, t18960: f64, t18963: f64, t18966: f64, t19077: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63533 = t2439 * t6132;
    let t63538 = t2439 * t6135;
    let t63541 = t698 * t19013;
    let t63543 = t698 * t19016;
    let t63545 = t2439 * t6138;
    let t63547 = t698 * t18960;
    let t63549 = t698 * t18963;
    let t63551 = t698 * t18966;
    let t63610 = t19077 * t914;
    (t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610)
}
