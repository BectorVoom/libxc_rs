//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1191/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1191(t125401: f64, t125440: f64, t125481: f64, t125526: f64, t125943: f64, t127333: f64, t127367: f64, t127409: f64, t1921: f64, t8602: f64, t2045: f64, t7939: f64) -> (f64, f64, f64) {
    let t127412 = t125401 + t125440 + t125481 + t125526 + t125943 + t127333 + t127367 + t127409;
    let t127416 = t8602 * t1921;
    let t127421 = t7939 * t2045;
    (t127412, t127416, t127421)
}
