//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 721/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk721(t3140: f64, t7690: f64, t131: f64, t1310: f64, t14148: f64, t25987: f64, t7351: f64, t12200: f64, t2044: f64, t321: f64, t7554: f64, t212: f64, t28: f64, t3144: f64, t4071: f64, t672: f64) -> (f64, f64, f64, f64) {
    let t70279 = t7690 * t3140;
    let t70316 = t14148 * t7351 * t131 * t1310 * t25987;
    let t70320 = t12200 * t2044 * t7554 * t321;
    let t70328 = t672 * t212 * t4071 * t28 * t3144;
    (t70279, t70316, t70320, t70328)
}
