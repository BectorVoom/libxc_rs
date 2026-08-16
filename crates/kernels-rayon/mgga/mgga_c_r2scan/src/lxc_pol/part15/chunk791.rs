//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 791/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk791(t4968: f64, t2850: f64, t797: f64, t2266: f64, t481: f64, t1527: f64, t2788: f64, t4983: f64, t2461: f64, t879: f64, t4721: f64, t4964: f64, t4967: f64, t4972: f64, t4975: f64, t4979: f64, t4981: f64) -> (f64, f64, f64, f64) {
    let t6954 = 0.21687162600603479684e-1_f64 * t4968;
    let t6955 = t2850 * t797;
    let t6957 = t2266 * t6955 * t481;
    let t6958 = 6.0_f64 * t6957;
    let t6959 = t2788 * t1527;
    let t6960 = 0.10843581300301739842e-1_f64 * t6959;
    let t6961 = 48.0_f64 * t4983;
    let t6963 = 2.0_f64 * t879 * t2461;
    let t6964 = -t4721 + t4964 - t4967 - t6954 - t4972 + t4975 - t6958 - t6960 + t4979 + t4981 - t6961 + t6963;
    (t6954, t6960, t6961, t6964)
}
