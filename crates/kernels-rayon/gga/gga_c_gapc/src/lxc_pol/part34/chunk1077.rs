//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1077/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1077(t16181: f64, t8132: f64, t7591: f64, t8141: f64, t952: f64, t291: f64, t4043: f64, t959: f64, t1153: f64, t2417: f64, t6851: f64, t869: f64) -> (f64, f64, f64, f64, f64) {
    let t16182 = t8132 * t16181;
    let t16296 = t7591 * t952 * t8141;
    let t16403 = t4043 * t291 * t959;
    let t16404 = t2417 * t1153;
    let t16408 = t869 * t6851;
    (t16182, t16296, t16403, t16404, t16408)
}
