//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 851/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk851(t9862: f64, t9865: f64, t197: f64, t7460: f64, t1077: f64, t7843: f64, t3336: f64, t1081: f64, t2737: f64, t3418: f64, t7511: f64, t3421: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9866 = t9862 * t9865;
    let t9868 = t197 * t7460;
    let t9869 = t1077 * t9868;
    let t9871 = t197 * t7843;
    let t9872 = t3336 * t9871;
    let t9874 = t1081 * t2737;
    let t9876 = t3418 * t7511;
    let t9878 = t3421 * t7511;
    (t9866, t9869, t9872, t9874, t9876, t9878)
}
