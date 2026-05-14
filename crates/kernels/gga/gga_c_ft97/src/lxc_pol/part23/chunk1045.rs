//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1045/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1045<F: Float>(t31640: F, t871: F, t1248: F, t28859: F, t317: F, t5225: F, t24989: F, t193: F, t2665: F, t4969: F, t6217: F, t25413: F, t5408: F, t25412: F, t1466: F, t1506: F, t28874: F, t28961: F, t28964: F, t28990: F, t29008: F, t31340: F, t31344: F, t31348: F, t31352: F, t31358: F, t31360: F, t5305: F, t6216: F, t6963: F, t6967: F, t6972: F, t7028: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31641 = t871 * t31640;
    let t31643 = t28859 * t1248;
    let t31646 = t317 * t5225;
    let t31647 = t24989 * t31646;
    let t31648 = t193 * t31647;
    let t31653 = t2665 * t6217 * t4969;
    let t31657 = t25413 * t5408;
    let t31658 = t25412 * t31657;
    let t31661 = -t28874 / 9.0 - t28961 / 9.0 + t6963 * t7028 / 3.0 - t6216 * t31340 / 9.0 - t6216 * t31344 / 9.0 - t6216 * t31348 / 18.0 - t6216 * t31352 / 27.0 - t29008 * t6967 / 9.0 + 2.0 / 9.0 * t28964 - 4.0 * t31358 + 4.0 * t31360 - 2.0 * t31641 - 4.0 * t31643 + t28990 / 27.0 + t1466 * t31648 - 2.0 / 3.0 * t6963 * t6972 + t6216 * t31653 / 9.0 - t5305 * t1506 + 2.0 / 9.0 * t6216 * t31658;
    (t31641, t31643, t31646, t31647, t31648, t31653, t31657, t31658, t31661)
}
