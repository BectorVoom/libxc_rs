//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 977/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk977(t635: f64, t9954: f64, t146: f64, t2156: f64, t112: f64, t115: f64, t6944: f64, t616: f64, t745: f64, t2359: f64, t4037: f64, t987: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9955 = t9954 * t635;
    let t9960 = t146 * t2156;
    let t9961 = t9960 * t112;
    let t10004 = t6944 * t115;
    let t10050 = t745 * t616;
    let t10109 = t2359 * t4037;
    let t10126 = t997 * t987;
    (t9955, t9961, t10004, t10050, t10109, t10126)
}
