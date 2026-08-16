//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 832/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk832(t285: f64, t7906: f64, t24: f64, t2629: f64, t862: f64, t2634: f64, t6541: f64, t865: f64, t322: f64, t7253: f64, t7256: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7907 = t7906 * t285;
    let t7914 = t24 * t2629;
    let t7915 = t862 * t7914;
    let t7917 = t24 * t2634;
    let t7918 = t862 * t7917;
    let t7920 = t865 * t6541;
    let t7921 = t322 * t7920;
    let t7924 = t7253 * t7256;
    let t7925 = t7924 * t6534;
    (t7907, t7914, t7915, t7917, t7918, t7920, t7921, t7924, t7925)
}
