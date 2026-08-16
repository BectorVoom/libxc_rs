//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 531/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk531(t1008: f64, t1086: f64, t1092: f64, t1098: f64, t1005: f64, t1103: f64, t1108: f64, t1113: f64, t952: f64, t957: f64, t935: f64, t940: f64, t950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3306 = t1008 * t1086;
    let t3308 = t1008 * t1092;
    let t3310 = t1008 * t1098;
    let t3312 = t1005 * t1103;
    let t3314 = t1005 * t1108;
    let t3316 = t1005 * t1113;
    let t3324 = t952 * t957;
    let t3326 = t935 * t957;
    let t3328 = t940 * t950;
    (t3306, t3308, t3310, t3312, t3314, t3316, t3324, t3326, t3328)
}
