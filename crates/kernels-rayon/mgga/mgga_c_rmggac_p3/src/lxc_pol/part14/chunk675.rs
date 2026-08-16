//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 675/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk675(t338: f64, t8794: f64, t118: f64, t1614: f64, t665: f64, t321: f64, t8936: f64, t797: f64, t8884: f64, t5148: f64, t8621: f64, t5259: f64, t8649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8957 = t338 * t8794;
    let t8958 = t118 * t8957;
    let t8960 = t665 * t1614;
    let t8963 = t8936 * t321;
    let t8966 = t797 * t8884;
    let t8971 = t5148 * t8621;
    let t8973 = t5259 * t8649;
    (t8957, t8958, t8960, t8963, t8966, t8971, t8973)
}
