//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 808/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk808<F: Float>(t2365: F, t28652: F, t6111: F, t40820: F, t900: F, t28973: F, t28028: F, t959: F, t10024: F, t10037: F, t22624: F, t7427: F, t9438: F) -> (F, F, F, F, F) {
    let t41337 = t6111 * t2365 * t28652;
    let t41339 = t900 * t40820;
    let t41340 = t28973 * t41339;
    let t41342 = t28028 * t959;
    let t41405 = t10037 * t10024;
    let t41408 = t7427 * t9438 * t22624;
    (t41337, t41340, t41342, t41405, t41408)
}
