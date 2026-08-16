//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2155/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2155(t1174: f64, t5045: f64, t698: f64, t3540: f64, t4966: f64, t11647: f64, t1744: f64, t3247: f64, t475: f64, t15032: f64, t3576: f64, t11713: f64, t11716: f64, t53081: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53270 = t1174 * t698 * t5045;
    let t53271 = t53270 / 432.0_f64;
    let t53272 = t4966 * t3540;
    let t53273 = t53272 / 4608.0_f64;
    let t53274 = t1744 * t11647;
    let t53298 = t475 * t3247;
    let t53322 = t15032 * t3576;
    let t53336 = t11713 * t11716 * t53081;
    (t53271, t53273, t53274, t53298, t53322, t53336)
}
