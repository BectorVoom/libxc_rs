//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 617/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk617(t1282: f64, t342: f64, t103: f64, t37: f64, t28: f64, t39: f64, t247: f64, t1227: f64, t361: f64, t38: f64, t61: f64, t939: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3494 = t1282 * t342;
    let t3500 = 1.0_f64 / t37 / t103 / 4.0_f64;
    let t3501 = param_hyb_omega_0 * t3500;
    let t3502 = t39 * t28;
    let t3505 = 1.9486833333333333_f64 * t3501 * t3502 * t247;
    let t3508 = 17.53815_f64 * t38 * t361 * t1227;
    let t3509 = t61 * t939;
    (t3494, t3500, t3501, t3502, t3505, t3508, t3509)
}
