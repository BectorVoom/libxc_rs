//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 939/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk939(t34976: f64, t40145: f64, t4550: f64, t495: f64, t8440: f64, t35039: f64, t39851: f64, t498: f64, t16504: f64, t321: f64, t333: f64, t3369: f64) -> (f64, f64, f64, f64) {
    let t40149 = t40145 * t34976 * t8440 * t4550 * t495;
    let t40154 = t39851 * t35039 * t8440 * t4550 * t498;
    let t40159 = t39851 * t16504 * t8440 * t4550 * t321;
    let t40164 = t39851 * t3369 * t8440 * t4550 * t333;
    (t40149, t40154, t40159, t40164)
}
