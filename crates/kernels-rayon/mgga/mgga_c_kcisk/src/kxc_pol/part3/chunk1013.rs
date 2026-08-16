//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1013/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1013(t1568: f64, t4416: f64, t12261: f64, t1592: f64, t535: f64, t4420: f64, t4419: f64, t4498: f64, t4377: f64, t14922: f64, t14925: f64, t14930: f64, t14937: f64, t1580: f64, t1593: f64, t4370: f64, t4378: f64, t4388: f64, t4393: f64, t4397: f64, t4510: f64) -> f64 {
    let t14940 = t1568 * t4416;
    let t14942 = t12261 * t1592;
    let t14943 = t535 * t14942;
    let t14945 = t1568 * t4420;
    let t14947 = t4419 * t4498;
    let t14948 = t535 * t14947;
    let t14956 = t4419 * t4377;
    let t14957 = t535 * t14956;
    let t14959 = 0.89953943580886586067e-2_f64 * t14922 + 0.11993859144118211476e-1_f64 * t14925 + 0.2698618307426597582e-1_f64 * t4397 * t4388 + 0.89953943580886586067e-2_f64 * t1580 * t14930 + 0.35981577432354634427e-1_f64 * t4397 * t4393 + 0.27985671336275826777e-1_f64 * t1580 * t14937 - 0.17990788716177317213e-1_f64 * t14940 + 0.17990788716177317213e-1_f64 * t14943 - 0.53972366148531951639e-1_f64 * t14945 - 0.2698618307426597582e-1_f64 * t14948 - 0.8095854922279792746e-1_f64 * t4370 * t1593 + 0.16191709844559585492e0_f64 * t1568 * t4378 - 0.7915947035118019574e0_f64 * t4510 * t1593 + 0.53972366148531951639e-1_f64 * t14957;
    t14959
}
