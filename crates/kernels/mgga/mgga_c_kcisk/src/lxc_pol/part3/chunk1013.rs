//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1013/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1013<F: Float>(t1568: F, t4416: F, t12261: F, t1592: F, t535: F, t4420: F, t4419: F, t4498: F, t4377: F, t14922: F, t14925: F, t14930: F, t14937: F, t1580: F, t1593: F, t4370: F, t4378: F, t4388: F, t4393: F, t4397: F, t4510: F) -> F {
    let t14940 = t1568 * t4416;
    let t14942 = t12261 * t1592;
    let t14943 = t535 * t14942;
    let t14945 = t1568 * t4420;
    let t14947 = t4419 * t4498;
    let t14948 = t535 * t14947;
    let t14956 = t4419 * t4377;
    let t14957 = t535 * t14956;
    let t14959 = F::new(0.89953943580886586067e-2) * t14922 + F::new(0.11993859144118211476e-1) * t14925 + F::new(0.2698618307426597582e-1) * t4397 * t4388 + F::new(0.89953943580886586067e-2) * t1580 * t14930 + F::new(0.35981577432354634427e-1) * t4397 * t4393 + F::new(0.27985671336275826777e-1) * t1580 * t14937 - F::new(0.17990788716177317213e-1) * t14940 + F::new(0.17990788716177317213e-1) * t14943 - F::new(0.53972366148531951639e-1) * t14945 - F::new(0.2698618307426597582e-1) * t14948 - F::new(0.8095854922279792746e-1) * t4370 * t1593 + F::new(0.16191709844559585492e0) * t1568 * t4378 - F::new(0.7915947035118019574e0) * t4510 * t1593 + F::new(0.53972366148531951639e-1) * t14957;
    t14959
}
