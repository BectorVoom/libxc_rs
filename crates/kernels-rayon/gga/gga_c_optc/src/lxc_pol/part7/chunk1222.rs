//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1222/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1222(t2595: f64, t9: f64, t2263: f64, t2640: f64, t7484: f64, t2270: f64, t3813: f64, t7899: f64, t889: f64, t2613: f64, t2620: f64, t24985: f64, t329: f64) -> (f64, f64, f64, f64, f64) {
    let t25217 = t9 * t2595;
    let t25218 = t25217 * t2263;
    let t25220 = t2640 * t25218 * t7484;
    let t25227 = t3813 * t2270;
    let t25237 = t7899 * t889;
    let t25239 = t2613 * t2620;
    let t25243 = t329 * t24985;
    (t25220, t25227, t25237, t25239, t25243)
}
