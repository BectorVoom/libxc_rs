//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 699/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk699(t236: f64, t9211: f64, t9210: f64, t3351: f64, t321: f64, t618: f64, t7248: f64, t2313: f64, t7715: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9212 = t236 * t9211;
    let t9213 = t9210 * t9212;
    let t9214 = t3351 * t9213;
    let t9216 = t618 * t321;
    let t9217 = t236 * t9216;
    let t9218 = t7248 * t9217;
    let t9219 = t3351 * t9218;
    let t9221 = t2313 * t7715;
    let t9222 = t9221 * t674;
    (t9213, t9214, t9216, t9218, t9219, t9221, t9222)
}
