//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 796/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk796(t6230: f64, t851: f64, t166: f64, t161: f64, t6307: f64, t6309: f64, t6311: f64, t6313: f64, t6315: f64, t6317: f64, t6319: f64, t6321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7442 = t6230 * t851;
    let t7443 = t166 * t7442;
    let t7445 = t161 * t7443 / 10.0_f64;
    let t7447 = 4.0_f64 / 45.0_f64 * t6307;
    let t7448 = 4.0_f64 / 45.0_f64 * t6309;
    let t7449 = 2.0_f64 / 45.0_f64 * t6311;
    let t7450 = 2.0_f64 / 27.0_f64 * t6313;
    let t7451 = 4.0_f64 / 45.0_f64 * t6315;
    let t7452 = 4.0_f64 / 45.0_f64 * t6317;
    let t7453 = 4.0_f64 / 45.0_f64 * t6319;
    let t7454 = 4.0_f64 / 45.0_f64 * t6321;
    (t7442, t7443, t7445, t7447, t7448, t7449, t7450, t7451, t7452, t7453, t7454)
}
