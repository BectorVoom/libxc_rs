//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1129/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1129(t1045: f64, t14282: f64, t3293: f64, t1728: f64, t3096: f64, t4642: f64, t2952: f64, t4637: f64, t4852: f64, t1762: f64, t3251: f64, t1071: f64, t1109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14283 = t14282 * t1045;
    let t14284 = t3293 * t14283;
    let t14287 = t1728 * t3096;
    let t14288 = t4642 * t14287;
    let t14291 = t4637 * t2952;
    let t14292 = t3293 * t14291;
    let t14295 = t4852 * t1045;
    let t14296 = t4642 * t14295;
    let t14299 = t3251 * t1762;
    let t14301 = t1109 * t1071;
    (t14283, t14284, t14287, t14288, t14291, t14292, t14295, t14296, t14299, t14301)
}
