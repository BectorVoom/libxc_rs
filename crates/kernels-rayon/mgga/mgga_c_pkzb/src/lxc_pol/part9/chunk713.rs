//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 713/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk713(t1476: f64, t4932: f64, t1475: f64, t475: f64, t574: f64, t474: f64, t49: f64, t4902: f64, t55: f64, t47: f64, t82: f64, t1489: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4933 = t1476 * t4932;
    let t4934 = t1475 * t4933;
    let t4936 = t475 * t574;
    let t4937 = t474 * t4936;
    let t4939 = t49 * t4902;
    let t4941 = 1.0_f64/pow_3_2(t55);
    let t4942 = t4941 * t47;
    let t4943 = t4942 * t82;
    let t4945 = t1489 * t4933;
    (t4934, t4936, t4937, t4939, t4942, t4943, t4945)
}
