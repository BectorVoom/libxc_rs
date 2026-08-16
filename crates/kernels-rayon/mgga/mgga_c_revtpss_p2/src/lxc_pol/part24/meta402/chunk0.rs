//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1337/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1337(t40196: f64, t760: f64, t10696: f64, t73: f64, t138: f64, t785: f64, t9302: f64, t234: f64, t39545: f64, t685: f64, t875: f64, t2778: f64, t39515: f64) -> (f64, f64, f64, f64, f64) {
    let t40198 = 0.35089341735807877242e1_f64 * t760 * t40196;
    let t40231 = t73 * t10696;
    let t40270 = t138 * t9302 * t785;
    let t40294 = 0.65457331274007190912e-5_f64 * t39545 * t234 * t875 * t685;
    let t40314 = 0.11564373972601816912e-1_f64 * t39515 * t2778;
    (t40198, t40231, t40270, t40294, t40314)
}
