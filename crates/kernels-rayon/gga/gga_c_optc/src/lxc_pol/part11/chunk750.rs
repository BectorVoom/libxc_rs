//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 750/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk750(t3466: f64, t624: f64, t155: f64, t6990: f64, t635: f64, t146: f64, t2156: f64, t112: f64, t1294: f64, t7022: f64, t115: f64, t6944: f64) -> (f64, f64, f64, f64, f64) {
    let t9917 = t3466 * t624;
    let t9954 = t155 * t6990;
    let t9955 = t9954 * t635;
    let t9960 = t146 * t2156;
    let t9961 = t9960 * t112;
    let t10002 = t7022 * t1294;
    let t10004 = t6944 * t115;
    (t9917, t9955, t9961, t10002, t10004)
}
