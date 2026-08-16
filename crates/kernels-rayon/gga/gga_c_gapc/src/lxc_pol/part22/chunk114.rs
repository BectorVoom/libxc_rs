//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 114/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk114(t321: f64, t322: f64, t326: f64, t334: f64, t31: f64, t4: f64, t79: f64) -> (f64, f64, f64) {
    let t337 = 1.0_f64 + 0.13900948042322754167e-2_f64 * t321 * t322 - 0.57970906942607043474e-5_f64 * t326 * t334;
    let t338 = 1.0_f64 / t337;
    let t344 = 0.11073577833333333333e-2_f64 * t4 * t79 * t31;
    (t337, t338, t344)
}
