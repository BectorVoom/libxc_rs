//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2022/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2022(t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t25059: f64, t6562: f64, t794: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64) -> (f64, f64, f64, f64, f64) {
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86928 = t6562 * t794 * t25059;
    let t86929 = 0.82246703342411321824e-2_f64 * t86928;
    let t86940 = t6562 * t82133 * t7488;
    let t86941 = 0.82246703342411321824e-2_f64 * t86940;
    let t86942 = t6547 * t25225;
    (t86911, t86916, t86929, t86941, t86942)
}
