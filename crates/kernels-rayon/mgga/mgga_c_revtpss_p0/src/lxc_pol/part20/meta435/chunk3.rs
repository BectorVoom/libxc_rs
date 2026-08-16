//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1642/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642(t1222: f64, t13028: f64, t140: f64, t13026: f64, t43776: f64, t3700: f64, t697: f64, t43750: f64, t43757: f64, t43759: f64, t43761: f64, t43965: f64, t43970: f64, t43980: f64, t43982: f64, t44011: f64, t44014: f64, t44021: f64) -> (f64, f64, f64, f64) {
    let t44972 = t1222 * t140 * t13028;
    let t44974 = t13026 * t43776;
    let t44980 = t1222 * t697 * t3700;
    let t44982 = -t43750 + t43757 - t43759 - t43761 - t43965 - t43970 - t43980 + t43982 + t44011 + t44014 - t44021;
    (t44972, t44974, t44980, t44982)
}
