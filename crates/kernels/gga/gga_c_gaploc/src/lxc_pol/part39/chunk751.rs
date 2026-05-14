//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 751/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk751<F: Float>(t12078: F, t1397: F, t12323: F, t747: F, t1959: F, t3730: F, t3720: F, t723: F, t701: F, t1: F, t106: F, t12161: F, t316: F, t12206: F, t783: F, t835: F) -> (F, F, F, F, F, F, F, F) {
    let t38770 = t1397 * t12078;
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    let t38907 = t3720 * t723;
    let t38912 = t3720 * t701;
    let t38947 = t12161 * t1 * t106 * t316;
    let t38950 = t12206 * t783;
    let t38961 = t835 * t12161;
    (t38770, t38885, t38892, t38907, t38912, t38947, t38950, t38961)
}
