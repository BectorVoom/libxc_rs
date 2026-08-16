//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2613/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2613(t11801: f64, t5005: f64, t15032: f64, t3576: f64, t11713: f64, t11716: f64, t53081: f64, t11786: f64, t5024: f64, t3032: f64, t52434: f64, t3505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53291 = t5005 * t11801;
    let t53322 = t15032 * t3576;
    let t53336 = t11713 * t11716 * t53081;
    let t53360 = t5024 * t11786;
    let t53371 = t52434 * t3032;
    let t53372 = t53371 * t3505;
    (t53291, t53322, t53336, t53360, t53371, t53372)
}
