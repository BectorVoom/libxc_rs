//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 952/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk952(t262: f64, t40901: f64, t35879: f64, t321: f64, t8708: f64, t7844: f64, t36250: f64, t38565: f64, t39693: f64, t7785: f64, t35824: f64, t39045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40902 = t262 * t40901;
    let t40903 = t35879 * t40902;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    let t40909 = t36250 * t38565;
    let t40911 = t7785 * t39693;
    let t40913 = t35824 * t39045;
    (t40902, t40903, t40905, t40906, t40907, t40909, t40911, t40913)
}
