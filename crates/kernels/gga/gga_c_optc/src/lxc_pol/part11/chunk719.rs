//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 719/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk719<F: Float>(t3466: F, t624: F, t155: F, t6990: F, t635: F, t146: F, t2156: F, t112: F, t1294: F, t7022: F, t115: F, t6944: F, t1287: F, t7030: F, t3563: F, t732: F) -> (F, F, F, F, F, F, F) {
    let t9917 = t3466 * t624;
    let t9954 = t155 * t6990;
    let t9955 = t9954 * t635;
    let t9960 = t146 * t2156;
    let t9961 = t9960 * t112;
    let t10002 = t7022 * t1294;
    let t10004 = t6944 * t115;
    let t10008 = t7030 * t1287;
    let t10036 = t732 * t3563;
    (t9917, t9955, t9961, t10002, t10004, t10008, t10036)
}
