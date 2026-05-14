//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 860/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk860<F: Float>(t1129: F, t15692: F, t1128: F, t3374: F, t278: F, t3375: F, t1099: F, t259: F, t1101: F, t15428: F, t15432: F, t15436: F, t15492: F, t15498: F, t15627: F, t282: F, t3368: F, t3373: F, t3377: F, t3423: F) -> (F, F, F, F, F, F) {
    let t15693 = t15692 * t1129;
    let t15696 = t3374 * t1128;
    let t15698 = 1.0 / t3375 / t278;
    let t15699 = t15696 * t15698;
    let t15703 = t1099 * t1099;
    let t15704 = 1.0 / t15703;
    let t15705 = t259 * t15704;
    let t15710 = -0.69644166666666666665e-2 * t15428 - 0.27857666666666666666e-1 * t15432 + 0.41786499999999999999e-1 * t15436 + 0.223494e0 * t15492 * t3377 - 0.579e0 * t3368 * t3423 + 0.579e0 * t1101 * t15498 + 0.579e0 * t3368 * t3377 + 0.223494e0 * t3373 * t15498 - 0.193e0 * t1101 * t15693 - 0.223494e0 * t3373 * t15699 + t15627 * t282 - 0.43134342e-1 * t15705 * t15699 - 0.386e0 * t1101 * t15699;
    (t15696, t15698, t15703, t15704, t15705, t15710)
}
