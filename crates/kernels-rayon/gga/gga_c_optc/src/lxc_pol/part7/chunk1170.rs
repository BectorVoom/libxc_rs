//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1170/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1170(t23682: f64, t23685: f64, t23660: f64, t23664: f64, t23667: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64, t23928: f64, t23931: f64, t23933: f64, t23936: f64, t23938: f64) -> f64 {
    let t24287 = 0.31003950617283950618e1_f64 * t23682;
    let t24288 = 0.13388493827160493828e1_f64 * t23685;
    let t24294 = 0.23917333333333333333e1_f64 * t23660 - 0.295764e1_f64 * t23664 + 0.65725333333333333332e0_f64 * t23667 + 0.71752000000000000001e1_f64 * t23670 - 0.79724444444444444444e0_f64 * t23673 - 0.19931111111111111111e1_f64 * t23676 - 0.107628e2_f64 * t23679 + t24287 + t24288 + 0.1898925e1_f64 * t23928 + 0.85451625e1_f64 * t23931 - 0.379785e1_f64 * t23933 - 0.46074375e0_f64 * t23936 + 0.614325e0_f64 * t23938;
    t24294
}
