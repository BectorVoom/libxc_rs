//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3174/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3174(t12469: f64, t1737: f64, t3362: f64, t462: f64, t2439: f64, t5101: f64, t16870: f64, t698: f64, t1729: f64, t9303: f64, t16894: f64, t16897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58005 = t1737 * t12469;
    let t58027 = t462 * t3362;
    let t58145 = t2439 * t5101;
    let t58147 = t698 * t16870;
    let t58153 = t9303 * t1729;
    let t58158 = t698 * t16894;
    let t58160 = t698 * t16897;
    (t58005, t58027, t58145, t58147, t58153, t58158, t58160)
}
