//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1301/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1301(t11291: f64, t23726: f64, t1616: f64, t2011: f64, t3721: f64, t3659: f64, t4915: f64, t3449: f64, t15430: f64, t11298: f64, t4908: f64, t11294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36055 = 12.0_f64 * t23726 * t11291;
    let t36058 = 2.0_f64 * t1616 * t3721 * t2011;
    let t36067 = 6.0_f64 * t4915 * t3659 * t2011;
    let t36068 = t3449 * t3449;
    let t36072 = 2.0_f64 * t15430 * t3659;
    let t36074 = 4.0_f64 * t4908 * t11298;
    let t36078 = 8.0_f64 * t4908 * t11294;
    (t36055, t36058, t36067, t36068, t36072, t36074, t36078)
}
