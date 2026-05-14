//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1214/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1214<F: Float>(t135: F, t15206: F, t60: F, t3375: F, t15698: F, t3374: F, t1097: F, t15704: F, t1100: F, t15627: F, t15703: F, t259: F, t281: F, t3366: F, t3372: F, t3435: F, t3441: F) -> (F, F, F, F, F, F, F, F) {
    let t43655 = t60 / t15206 / t135;
    let t43669 = t3375 * t3375;
    let t43670 = 1.0 / t43669;
    let t43674 = t3374 * t15698;
    let t43680 = t1097 * t15704;
    let t43683 = t15627 * t1100;
    let t43939 = t259 / t15703 / t281;
    let t43982 = t3366 * t3372;
    let t44167 = t3435 * t3441;
    (t43655, t43670, t43674, t43680, t43683, t43939, t43982, t44167)
}
