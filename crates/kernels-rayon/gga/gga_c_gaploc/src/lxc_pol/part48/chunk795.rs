//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 795/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk795(t20671: f64, t31047: f64, t34814: f64, t26984: f64, t9294: f64, t12953: f64, t31054: f64, t12986: f64, t2464: f64, t2487: f64, t3177: f64, t35091: f64, t9272: f64) -> (f64, f64, f64, f64, f64) {
    let t42187 = t31047 * t20671 * t34814;
    let t42189 = t26984 * t9294;
    let t42199 = t31054 * t12953;
    let t42202 = t2487 * t2464 * t12986;
    let t42226 = t9272 * t35091 * t3177;
    (t42187, t42189, t42199, t42202, t42226)
}
