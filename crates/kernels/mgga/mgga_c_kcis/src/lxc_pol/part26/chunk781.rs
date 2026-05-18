//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 781/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk781<F: Float>(t154: F, t8750: F, t808: F, t152: F, t8536: F, t8538: F, t143: F, t8747: F, t21: F, t2469: F, t2553: F, t2459: F, t88: F) -> (F, F, F, F, F, F) {
    let t9061 = t154 * t8750;
    let t9062 = t808 * t9061;
    let t9064 = t152 * t8536;
    let t9065 = t154 * t8538;
    let t9066 = t9064 * t9065;
    let t9070 = t8747 * t143;
    let t9074 = t2553 * t21 * t2469;
    let t9077 = t88 * t2459;
    (t9062, t9064, t9066, t9070, t9074, t9077)
}
