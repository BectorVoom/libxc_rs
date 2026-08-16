//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1174/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1174(t1165: f64, t1180: f64, t1181: f64, t13656: f64, t1439: f64, t15995: f64, t16407: f64, t16409: f64, t16415: f64, t16417: f64, t16421: f64, t16423: f64, t1866: f64, t1899: f64, t335: f64, t3396: f64, t4533: f64, t4643: f64, t4680: f64, t4752: f64, t4838: f64, t530: f64, t5902: f64, t930: f64, t960: f64) -> f64 {
    let t21290 = t335 * t960 * t530 * t4838 / 24.0_f64 + t335 * t13656 * t1866 / 24.0_f64 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t15995 * t1439 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t4643 * t4752 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t4643 * t4533 + 0.34299214494455789578e-2_f64 * t16407 + 0.42874018118069736972e-3_f64 * t1180 * t1165 * t1899 * t930 + 0.13719685797782315831e-1_f64 * t3396 * t4680 * t5902 - 0.64025200389650807212e-1_f64 * t16409 - 0.17149607247227894789e-1_f64 * t16415 - 0.64025200389650807212e-1_f64 * t16417 + 0.13719685797782315831e-1_f64 * t16421 + 0.64025200389650807212e-1_f64 * t16423;
    t21290
}
