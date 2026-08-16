//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 855/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk855(t1627: f64, t262: f64, t69296: f64, t1632: f64, t1635: f64, t25636: f64, t3068: f64, t1624: f64, t3076: f64, t2044: f64, t25518: f64, t556: f64, t69199: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75346 = t262 * t1627;
    let t75347 = t69296 * t75346;
    let t75351 = t262 * t1632;
    let t75352 = t69296 * t75351;
    let t75355 = t262 * t1635;
    let t75356 = t25636 * t3068 * t75355;
    let t75359 = t3076 * t1624;
    let t75360 = t25518 * t2044 * t75359;
    let t75362 = t69199 * t556;
    (t75346, t75347, t75351, t75352, t75355, t75356, t75359, t75360, t75362)
}
