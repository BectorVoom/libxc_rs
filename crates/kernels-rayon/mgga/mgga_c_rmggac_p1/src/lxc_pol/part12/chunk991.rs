//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 991/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk991(t39697: f64, t7788: f64, t333: f64, t8712: f64, t262: f64, t7829: f64, t35960: f64, t5145: f64, t649: f64, t26: f64, t7834: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40918 = t7788 * t39697;
    let t40920 = t8712 * t333;
    let t40921 = t262 * t40920;
    let t40922 = t7829 * t40921;
    let t40925 = t35960 * t649 * t5145;
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    (t40918, t40920, t40921, t40922, t40925, t40927, t40928)
}
