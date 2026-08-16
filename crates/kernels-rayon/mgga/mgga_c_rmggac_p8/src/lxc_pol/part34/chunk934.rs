//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 934/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk934(t76665: f64, t15478: f64, t16156: f64, t15504: f64, t73791: f64, t73797: f64, t73799: f64, t1971: f64, t3351: f64, t7262: f64, t9571: f64, t1986: f64, t2467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76666 = 0.12769379967989351819e-4_f64 * t76665;
    let t76667 = t16156 * t15478;
    let t76668 = 0.29795219925308487578e-4_f64 * t76667;
    let t76669 = t16156 * t15504;
    let t76670 = 0.99317399751028291929e-5_f64 * t76669;
    let t76671 = 0.19709219354514038085e-5_f64 * t73791;
    let t76673 = 0.2627895913935205078e-5_f64 * t73797;
    let t76674 = 0.2627895913935205078e-5_f64 * t73799;
    let t76678 = t3351 * t1971 * t7262 * t9571;
    let t76679 = 0.25538759935978703639e-4_f64 * t76678;
    let t76680 = t1986 * t2467;
    (t76666, t76668, t76670, t76671, t76673, t76674, t76679, t76680)
}
