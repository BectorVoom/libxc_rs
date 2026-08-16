//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 550/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk550(t31: f64, t357: f64, t2046: f64, t2050: f64, t2131: f64, t931: f64, t668: f64, t934: f64) -> (f64, f64, f64, f64) {
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7397 = t931 * t2131;
    let t7398 = 0.2363e1_f64 * t7397;
    let t7399 = t934 * t668;
    (t7393, t7395, t7398, t7399)
}
