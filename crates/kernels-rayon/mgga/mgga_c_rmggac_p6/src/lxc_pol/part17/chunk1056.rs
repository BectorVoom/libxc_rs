//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1056/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1056(t2298: f64, t26370: f64, t17859: f64, t9051: f64, t9055: f64, t9096: f64, t9138: f64, t2310: f64, t38472: f64, t9190: f64, t9194: f64, t10090: f64, t16156: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47340 = t26370 * t2298;
    let t47345 = t17859 * t9051;
    let t47347 = t17859 * t9055;
    let t47349 = t17859 * t9096;
    let t47351 = t17859 * t9138;
    let t47353 = t38472 * t2310;
    let t47355 = t17859 * t9190;
    let t47357 = t17859 * t9194;
    let t47359 = t16156 * t10090;
    (t47340, t47345, t47347, t47349, t47351, t47353, t47355, t47357, t47359)
}
