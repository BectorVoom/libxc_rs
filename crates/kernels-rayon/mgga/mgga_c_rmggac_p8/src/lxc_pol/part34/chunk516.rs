//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 516/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk516(t14150: f64, t7351: f64, t14148: f64, t262: f64, t352: f64, t3068: f64, t10570: f64, t384: f64, t464: f64, t220: f64, t1966: f64, t209: f64, t26: f64, t476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14151 = t7351 * t14150;
    let t14152 = t14148 * t14151;
    let t14154 = t262 * t352;
    let t14155 = t3068 * t14154;
    let t14156 = t10570 * t14155;
    let t14161 = t464 * t384;
    let t14162 = t14161 * t220;
    let t14163 = t1966 * t14162;
    let t14165 = t26 * t476 * t209;
    (t14151, t14152, t14154, t14155, t14156, t14161, t14162, t14163, t14165)
}
