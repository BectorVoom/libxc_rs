//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 682/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk682<F: Float>(t2366: F, t38276: F, t12000: F, t158: F, t203: F, t1: F, t544: F, t1359: F, t3689: F, t12078: F, t1397: F, t12323: F, t747: F, t1959: F, t3730: F, t3720: F, t723: F) -> (F, F, F, F, F, F, F, F) {
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    let t38413 = t203 * t12000;
    let t38486 = t544 * t38285 * t1;
    let t38674 = t1359 * t3689;
    let t38770 = t1397 * t12078;
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    let t38907 = t3720 * t723;
    (t38281, t38413, t38486, t38674, t38770, t38885, t38892, t38907)
}
