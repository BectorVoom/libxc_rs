//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 825/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk825(t4617: f64, t507: f64, t1622: f64, t1986: f64, t1679: f64, t7197: f64, t34760: f64, t9221: f64, t352: f64, t8915: f64, t5148: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40724 = t507 * t4617;
    let t40731 = t1986 * t1622;
    let t40759 = t1679 * t7197;
    let t40771 = t9221 * t34760;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40805 = t8915 * t333;
    (t40724, t40731, t40759, t40771, t40802, t40803, t40805)
}
