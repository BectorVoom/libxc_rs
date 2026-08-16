//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 631/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk631(t15354: f64, t15357: f64, t15364: f64, t15368: f64, t15377: f64, t15380: f64, t15389: f64, t15392: f64, t15400: f64, t15412: f64, t3285: f64, t534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15920 = 0.52557918278704101561e-6_f64 * t15354;
    let t15921 = 0.52557918278704101561e-6_f64 * t15357;
    let t15922 = 0.58171619854173713844e-5_f64 * t15364;
    let t15923 = 0.17451485956252114153e-4_f64 * t15368;
    let t15924 = 0.58171619854173713844e-5_f64 * t15377;
    let t15925 = 0.58171619854173713844e-5_f64 * t15380;
    let t15927 = 0.35038612185802734374e-6_f64 * t15389;
    let t15928 = 0.35038612185802734374e-6_f64 * t15392;
    let t15929 = 0.72714524817717142305e-5_f64 * t15400;
    let t15930 = 0.58171619854173713844e-5_f64 * t15412;
    let t15931 = t534 * t3285;
    (t15920, t15921, t15922, t15923, t15924, t15925, t15927, t15928, t15929, t15930, t15931)
}
