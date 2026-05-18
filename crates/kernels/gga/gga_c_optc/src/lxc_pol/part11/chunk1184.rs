//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1184/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1184<F: Float>(t17697: F, t3117: F, t17893: F, t26940: F, t15690: F, t4314: F, t12075: F, t17662: F, t3116: F, t17855: F, t3104: F, t12026: F, t15254: F) -> (F, F, F, F, F, F, F) {
    let t53963 = t3117 * t17697;
    let t53972 = t26940 * t17893;
    let t53987 = t15690 * t4314;
    let t53995 = t3116 * t12075 * t17662;
    let t54066 = t3104 * t17855;
    let t54079 = t3117 * t17855;
    let t54105 = t12026 * t15254;
    (t53963, t53972, t53987, t53995, t54066, t54079, t54105)
}
