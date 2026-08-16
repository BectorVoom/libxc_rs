//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 851/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk851(t3502: f64, t8333: f64, t8337: f64, t3500: f64, t3510: f64, t61: f64, t1179: f64, t342: f64, t55: f64, t1276: f64, t1243: f64, t1180: f64, t361: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64) {
    let t8339 = 1.6239027777777777_f64 * param_hyb_omega_0 * t8333 * t3502 * t8337;
    let t8346 = 0.16322666666666666_f64 * t61 * t3500 * t3510 * t8337;
    let t8352 = t55 * t1179 * t342;
    let t8353 = t1276 * t8352;
    let t8355 = t1243 * t8352;
    let t8357 = t1180 * t361;
    (t8339, t8346, t8353, t8355, t8357)
}
