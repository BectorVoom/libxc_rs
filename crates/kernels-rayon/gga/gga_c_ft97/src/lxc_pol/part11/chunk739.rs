//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 739/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk739(t2579: f64, t684: f64, t10007: f64, t2603: f64, t8392: f64, t3892: f64, t9853: f64, t3891: f64, t2526: f64, t713: f64, t729: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10008 = t2579 * t684;
    let t10009 = t10007 * t10008;
    let t10012 = t8392 * t2603;
    let t10014 = t3892 * t9853;
    let t10015 = t3891 * t10014;
    let t10018 = t2526 * t713;
    let t10020 = t729 * t762 * t10018;
    (t10008, t10009, t10012, t10014, t10015, t10018, t10020)
}
