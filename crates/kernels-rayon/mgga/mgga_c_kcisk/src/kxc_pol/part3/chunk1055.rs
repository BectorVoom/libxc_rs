//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1055/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1055(t1129: f64, t15692: f64, t1128: f64, t3374: f64, t278: f64, t3375: f64, t1099: f64, t259: f64, t1101: f64, t15428: f64, t15432: f64, t15436: f64, t15492: f64, t15498: f64, t15627: f64, t282: f64, t3368: f64, t3373: f64, t3377: f64, t3423: f64) -> f64 {
    let t15693 = t15692 * t1129;
    let t15696 = t3374 * t1128;
    let t15698 = 1.0_f64 / t3375 / t278;
    let t15699 = t15696 * t15698;
    let t15703 = t1099 * t1099;
    let t15704 = 1.0_f64 / t15703;
    let t15705 = t259 * t15704;
    let t15710 = -0.69644166666666666665e-2_f64 * t15428 - 0.27857666666666666666e-1_f64 * t15432 + 0.41786499999999999999e-1_f64 * t15436 + 0.223494e0_f64 * t15492 * t3377 - 0.579e0_f64 * t3368 * t3423 + 0.579e0_f64 * t1101 * t15498 + 0.579e0_f64 * t3368 * t3377 + 0.223494e0_f64 * t3373 * t15498 - 0.193e0_f64 * t1101 * t15693 - 0.223494e0_f64 * t3373 * t15699 + t15627 * t282 - 0.43134342e-1_f64 * t15705 * t15699 - 0.386e0_f64 * t1101 * t15699;
    t15710
}
