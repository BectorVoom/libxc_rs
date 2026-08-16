//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1356/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1356(t1128: f64, t8921: f64, t8923: f64, t1137: f64, t3843: f64, t1133: f64, t3152: f64, t7878: f64, t8960: f64, t8962: f64, t1122: f64, t1135: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26999 = t8921 * t1128 * t8923;
    let t27001 = t3843 * t1137;
    let t27002 = t1133 * t27001;
    let t27004 = t7878 * t3152;
    let t27005 = t1133 * t27004;
    let t27008 = t8960 * t1128 * t8962;
    let t27010 = t1135 * t1122;
    (t26999, t27001, t27002, t27004, t27005, t27008, t27010)
}
