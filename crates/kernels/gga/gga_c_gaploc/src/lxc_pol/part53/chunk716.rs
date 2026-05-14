//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 716/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk716<F: Float>(t28973: F, t41339: F, t28028: F, t959: F, t10024: F, t10037: F, t22624: F, t7427: F, t9438: F, t12651: F, t2684: F, t7354: F, t12652: F, t7416: F, t161: F, t165: F, t9688: F) -> (F, F, F, F, F, F, F) {
    let t41340 = t28973 * t41339;
    let t41342 = t28028 * t959;
    let t41405 = t10037 * t10024;
    let t41408 = t7427 * t9438 * t22624;
    let t41411 = t2684 * t7354 * t12651;
    let t41413 = t7416 * t12652;
    let t41416 = t161 * t165 * t9688;
    (t41340, t41342, t41405, t41408, t41411, t41413, t41416)
}
