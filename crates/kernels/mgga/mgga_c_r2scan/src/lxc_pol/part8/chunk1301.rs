//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1301/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1301<F: Float>(t25299: F, t7624: F, t6155: F, t29783: F, t6086: F, t6093: F, t2147: F, t30320: F, t22709: F, t5108: F, t9481: F, t25852: F, t5147: F, t5148: F, t9318: F, t20594: F, t2687: F, t7605: F) -> (F, F, F, F, F, F, F) {
    let t31074 = t25299 * t7624;
    let t31075 = t6155 * t31074;
    let t31083 = t6093 * t6086 * t29783;
    let t31086 = t2147 * t6086 * t30320;
    let t31092 = t5108 * t22709 * t9481;
    let t31095 = t25852 * t31074;
    let t31099 = t5147 * t5148 * t9318;
    let t31102 = t20594 * t2687 * t7605;
    (t31075, t31083, t31086, t31092, t31095, t31099, t31102)
}
