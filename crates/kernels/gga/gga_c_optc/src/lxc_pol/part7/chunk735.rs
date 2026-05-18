//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 735/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk735<F: Float>(t6869: F, t696: F, t2120: F, t2132: F, t127: F, t5: F, t6867: F, t675: F, t155: F, t158: F, t2078: F, t661: F) -> (F, F, F, F, F) {
    let t7048 = t696 * t6869;
    let t7051 = t2120 * t2132;
    let t7054 = t5 * t6867 * t127;
    let t7055 = t675 * t7054;
    let t7061 = t155 * t158 * t2078;
    let t7062 = t7061 * t661;
    (t7048, t7051, t7055, t7061, t7062)
}
