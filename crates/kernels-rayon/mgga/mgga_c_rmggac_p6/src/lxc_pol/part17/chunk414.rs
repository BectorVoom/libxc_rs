//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 414/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk414(t189: f64, t53: f64, t191: f64, t60: f64, t1346: f64, t49: f64, t288: f64, t325: f64, t504: f64, t507: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3985 = 1.0_f64 / t189 / t53;
    let t3998 = 1.0_f64 / t191 / t60;
    let t4035 = t1346 * t49;
    let t4036 = t4035 * t288;
    let t4041 = t504 * t325;
    let t4044 = t507 * t837;
    (t3985, t3998, t4035, t4036, t4041, t4044)
}
