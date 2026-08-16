//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1045/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1045(t40759: f64, t8646: f64, t2295: f64, t30174: f64, t5055: f64, t8884: f64, t34735: f64, t9705: f64, t36978: f64, t46511: f64, t34738: f64, t46515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47787 = t40759 * t8646;
    let t47795 = t30174 * t2295;
    let t47797 = t5055 * t8884;
    let t47800 = t34735 * t9705;
    let t47802 = t36978 * t46511;
    let t47804 = t34738 * t46515;
    (t47787, t47795, t47797, t47800, t47802, t47804)
}
