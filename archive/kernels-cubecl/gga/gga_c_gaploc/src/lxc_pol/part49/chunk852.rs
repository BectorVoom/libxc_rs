//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 852/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk852<F: Float>(t12064: F, t540: F, t1: F, t106: F, t12000: F, t192: F, t12078: F, t1397: F, t12323: F, t747: F, t1959: F, t3730: F) -> (F, F, F, F, F) {
    let t38688 = t12064 * t540;
    let t38759 = t12000 * t1 * t106 * t192;
    let t38770 = t1397 * t12078;
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    (t38688, t38759, t38770, t38885, t38892)
}
