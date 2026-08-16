//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 701/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk701(t10100: f64, t236: f64, t3352: f64, t7230: f64, t1707: f64, t511: f64, t3351: f64, t1704: f64, t234: f64, t681: f64, t2073: f64, t9877: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10101 = t236 * t10100;
    let t10102 = t3352 * t10101;
    let t10103 = t7230 * t10102;
    let t10105 = t511 * t1707;
    let t10106 = t3352 * t10105;
    let t10107 = t3351 * t10106;
    let t10112 = t234 * t1704;
    let t10113 = t10112 * t681;
    let t10120 = t2073 * t9877;
    (t10102, t10103, t10106, t10107, t10112, t10113, t10120)
}
