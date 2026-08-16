//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 367/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk367(t1564: f64, t379: f64, t5675: f64, t5674: f64, t1800: f64, t5635: f64, t1317: f64, t28: f64, t469: f64, t5617: f64, t1322: f64, t375: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5677 = t1564 * t5675 * t379;
    let t5678 = t5674 * t5677;
    let t5680 = t1800 * t5635;
    let t5682 = t1317 * t28 * t5680;
    let t5684 = t469 * t5617;
    let t5686 = t1317 * t28 * t5684;
    let t5689 = t89 * t375 * t1322;
    (t5677, t5678, t5680, t5682, t5684, t5686, t5689)
}
