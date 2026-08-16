//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 976/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk976(t4003: f64, t6843: f64, t10114: f64, t10117: f64, t10126: f64, t10129: f64, t14243: f64, t14252: f64, t1883: f64, t213: f64, t22009: f64, t22329: f64, t22333: f64, t22337: f64, t22353: f64, t22362: f64, t22366: f64, t22370: f64, t22374: f64, t22381: f64, t22964: f64, t546: f64, t5735: f64, t5745: f64, t5755: f64) -> f64 {
    let t23037 = t4003 * t6843;
    let t23041 = -0.58544643236296698113e-1_f64 * t22329 - 0.29272321618148349057e-1_f64 * t22333 - 0.29272321618148349057e-1_f64 * t22337 + 0.39029762157531132076e-1_f64 * t14243 + t10114 + 0.65854491829355115987e0_f64 * t213 * t546 * t22964 - t10117 - 0.16463622957338778996e-1_f64 * t22353 - t10126 - t10129 - 0.39029762157531132076e-1_f64 * t14252 - 0.32927245914677557992e-1_f64 * t22362 + 0.32927245914677557992e-1_f64 * t22366 + 0.16463622957338778996e-1_f64 * t22370 + 0.16463622957338778996e-1_f64 * t22374 + 0.29272321618148349057e-1_f64 * t22381 - 0.19756347548806534796e1_f64 * t5755 * t22009 * t1883 + 0.39512695097613069591e1_f64 * t5745 * t5735 * t23037;
    t23041
}
