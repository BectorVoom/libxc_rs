//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 917/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk917(t1005: f64, t3756: f64, t3652: f64, t3775: f64, t3657: f64, t1086: f64, t3670: f64, t1113: f64, t3700: f64, t3740: f64, t957: f64, t1163: f64, t1165: f64, t3439: f64, t4162: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14003 = t1005 * t3756;
    let t14005 = t3775 * t3652;
    let t14015 = t3775 * t3657;
    let t14017 = t3670 * t1086;
    let t14019 = t3700 * t1113;
    let t14022 = t3740 * t957;
    let t14044 = t1163 * t1165 * t3439 * t4162;
    (t14003, t14005, t14015, t14017, t14019, t14022, t14044)
}
