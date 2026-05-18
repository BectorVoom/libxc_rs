//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 767/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk767<F: Float>(t12323: F, t747: F, t1959: F, t3730: F, t3720: F, t723: F, t701: F, t12161: F, t325: F, t1858: F, t7290: F, t321: F) -> (F, F, F, F, F, F, F, F) {
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    let t38907 = t3720 * t723;
    let t38912 = t3720 * t701;
    let t38974 = t325 * t12161;
    let t39002 = t1858 * t3720;
    let t39040 = t7290 * t38907;
    let t39048 = t321 * t3720;
    (t38885, t38892, t38907, t38912, t38974, t39002, t39040, t39048)
}
