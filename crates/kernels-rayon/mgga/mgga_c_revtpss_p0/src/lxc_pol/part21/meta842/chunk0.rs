//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3154/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3154(t1145: f64, t141: f64, t56232: f64, t1729: f64, t9303: f64, t56153: f64, t16894: f64, t698: f64, t16897: f64, t16900: f64, t2439: f64, t5095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58151 = t141 * t1145 * t56232;
    let t58153 = t9303 * t1729;
    let t58156 = t141 * t1145 * t56153;
    let t58158 = t698 * t16894;
    let t58160 = t698 * t16897;
    let t58162 = t698 * t16900;
    let t58165 = t2439 * t5095;
    (t58151, t58153, t58156, t58158, t58160, t58162, t58165)
}
