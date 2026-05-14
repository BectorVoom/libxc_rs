//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 899/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk899<F: Float>(t14942: F, t535: F, t1568: F, t4420: F, t4419: F, t4498: F, t4377: F, t14922: F, t14925: F, t14930: F, t14937: F, t14940: F, t1580: F, t1593: F, t4370: F, t4378: F, t4388: F, t4393: F, t4397: F, t4510: F) -> (F,) {
    let t14943 = t535 * t14942;
    let t14945 = t1568 * t4420;
    let t14947 = t4419 * t4498;
    let t14948 = t535 * t14947;
    let t14956 = t4419 * t4377;
    let t14957 = t535 * t14956;
    let t14959 = 0.89953943580886586067e-2 * t14922 + 0.11993859144118211476e-1 * t14925 + 0.2698618307426597582e-1 * t4397 * t4388 + 0.89953943580886586067e-2 * t1580 * t14930 + 0.35981577432354634427e-1 * t4397 * t4393 + 0.27985671336275826777e-1 * t1580 * t14937 - 0.17990788716177317213e-1 * t14940 + 0.17990788716177317213e-1 * t14943 - 0.53972366148531951639e-1 * t14945 - 0.2698618307426597582e-1 * t14948 - 0.8095854922279792746e-1 * t4370 * t1593 + 0.16191709844559585492e0 * t1568 * t4378 - 0.7915947035118019574e0 * t4510 * t1593 + 0.53972366148531951639e-1 * t14957;
    (t14959,)
}
