//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1034/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1034(t236: f64, t3351: f64, t6412: f64, t9188: f64, t2305: f64, t39393: f64, t8497: f64, t8577: f64, t1734: f64, t498: f64, t7248: f64, t6415: f64) -> (f64, f64, f64, f64, f64) {
    let t47565 = t3351 * t9188 * t236 * t6412;
    let t47570 = t39393 * t2305;
    let t47572 = t8577 * t8497;
    let t47577 = t3351 * t7248 * t236 * t1734 * t498;
    let t47581 = t3351 * t9188 * t236 * t6415;
    (t47565, t47570, t47572, t47577, t47581)
}
