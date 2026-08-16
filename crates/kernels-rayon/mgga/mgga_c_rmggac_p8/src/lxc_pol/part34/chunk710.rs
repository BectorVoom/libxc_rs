//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 710/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk710(t3154: f64, t7921: f64, t14040: f64, t14367: f64, t14042: f64, t14115: f64, t68454: f64, t14147: f64, t14151: f64, t7348: f64, t1295: f64, t131: f64, t14148: f64, t25987: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69835 = 0.66211599834018861287e-4_f64 * t7921 * t3154;
    let t69836 = t14040 * t14367;
    let t69837 = t69836 * t14042;
    let t69839 = t68454 * t14115;
    let t69860 = t14147 * t7348 * t14151;
    let t69865 = t14148 * t7351 * t131 * t1295 * t25987;
    (t69835, t69836, t69837, t69839, t69860, t69865)
}
