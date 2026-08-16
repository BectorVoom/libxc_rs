//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 410/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk410(t930: f64, t941: f64, t189: f64, t53: f64, t191: f64, t60: f64, t356: f64, t934: f64, t1276: f64, t290: f64, t1288: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3981 = t941 * t930;
    let t3985 = 1.0_f64 / t189 / t53;
    let t3998 = 1.0_f64 / t191 / t60;
    let t4018 = t934 * t356;
    let t4025 = t290 * t1276;
    let t4028 = t68 * t1288;
    (t3981, t3985, t3998, t4018, t4025, t4028)
}
