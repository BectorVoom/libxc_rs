//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1026/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1026(t2019: f64, t2020: f64, t9750: f64, t2010: f64, t2012: f64, t6627: f64, t13283: f64, t2061: f64, t10082: f64, t236: f64, t321: f64, t3351: f64, t35155: f64) -> (f64, f64, f64, f64) {
    let t47439 = t2019 * t2020 * t9750;
    let t47442 = t2010 * t2012 * t6627;
    let t47445 = t13283 * t2061;
    let t47450 = t3351 * t35155 * t236 * t10082 * t321;
    (t47439, t47442, t47445, t47450)
}
