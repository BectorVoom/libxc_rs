//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 827/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk827(t275: f64, t9064: f64, t1679: f64, t7197: f64, t34760: f64, t9221: f64, t352: f64, t8915: f64, t5148: f64, t333: f64, t4669: f64, t128: f64, t30526: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40750 = 2.0_f64 * t275 * t9064;
    let t40759 = t1679 * t7197;
    let t40771 = t9221 * t34760;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40804 = 0.15965655602485078085e0_f64 * t40803;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40807 = 0.23948483403727617128e0_f64 * t40806;
    let t40823 = t30526 * t128;
    (t40750, t40759, t40771, t40802, t40804, t40805, t40807, t40823)
}
