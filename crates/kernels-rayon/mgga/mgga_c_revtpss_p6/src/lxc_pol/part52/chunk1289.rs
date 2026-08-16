//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1289/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1289(t2045: f64, t8113: f64, t122799: f64, t127515: f64, t127516: f64, t129014: f64, t129037: f64, t129060: f64, t129093: f64, t129112: f64, t1456: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t2111: f64, t2118: f64, t28235: f64, t28283: f64, t28945: f64, t3: f64, t32744: f64, t32782: f64, t34333: f64, t34369: f64, t575: f64, t5790: f64, t7319: f64, t7542: f64, t7956: f64, t8130: f64, t8734: f64) -> f64 {
    let t129118 = t8113 * t2045;
    let t129126 = t7319 * t8130 + t28945 * t2045 + t1456 * t34369 + t127515 + t122799 + t127516 + t1458 * (t129037 + t129060 + t129093 + t129112) + t5790 * t8734 + t1914 * t32782 + t129118 + t34333 * t1464 + t32744 * t1921 + t3 * t129014 * t575 + t28235 * t2118 + t2111 * t28283 + t7542 * t7956;
    t129126
}
