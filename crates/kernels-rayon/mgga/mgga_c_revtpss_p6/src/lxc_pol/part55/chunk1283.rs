//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1283/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1283(t116: f64, t34775: f64, t670: f64, t8885: f64, t125384: f64, t125386: f64, t125388: f64, t125390: f64, t129467: f64, t129470: f64, t2055: f64, t27060: f64, t28683: f64, t29427: f64, t29432: f64, t34446: f64, t7373: f64, t7586: f64, t7983: f64) -> (f64, f64, f64) {
    let t130929 = t34775 * t116;
    let t130932 = t8885 * t670;
    let t130946 = t129467 * t2055 + t129470 * t2055 + t27060 * t7983 + t28683 * t7586 + t29427 * t7373 + t29432 * t7983 + t34446 * t7373 + t125384 + t125386 + t125388 + t125390;
    (t130929, t130932, t130946)
}
