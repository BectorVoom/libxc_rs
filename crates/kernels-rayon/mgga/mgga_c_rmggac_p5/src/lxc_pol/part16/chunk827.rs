//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 827/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk827(t262: f64, t40897: f64, t7785: f64, t321: f64, t8708: f64, t7844: f64, t39693: f64, t39697: f64, t7788: f64, t333: f64, t8712: f64, t7829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40898 = t262 * t40897;
    let t40899 = t7785 * t40898;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    let t40911 = t7785 * t39693;
    let t40918 = t7788 * t39697;
    let t40920 = t8712 * t333;
    let t40921 = t262 * t40920;
    let t40922 = t7829 * t40921;
    (t40898, t40899, t40905, t40906, t40907, t40911, t40918, t40920, t40921, t40922)
}
