//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 909/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk909<F: Float>(t2029: F, t3500: F, t3466: F, t624: F, t155: F, t6990: F, t635: F, t146: F, t2156: F, t112: F, t115: F, t6944: F, t616: F, t745: F, t2359: F, t4037: F) -> (F, F, F, F, F, F, F) {
    let t9896 = t3500 * t2029;
    let t9917 = t3466 * t624;
    let t9954 = t155 * t6990;
    let t9955 = t9954 * t635;
    let t9960 = t146 * t2156;
    let t9961 = t9960 * t112;
    let t10004 = t6944 * t115;
    let t10050 = t745 * t616;
    let t10109 = t2359 * t4037;
    (t9896, t9917, t9955, t9961, t10004, t10050, t10109)
}
