//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1465/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465(t13222: f64, t16968: f64, t16673: f64, t842: f64, t13345: f64, t13365: f64, t1516: f64, t16914: f64, t16918: f64, t16924: f64, t16928: f64, t16932: f64, t16937: f64, t16940: f64, t16942: f64, t16946: f64, t16951: f64, t16954: f64, t16957: f64, t16961: f64, t16965: f64, t2571: f64, t2643: f64, t4172: f64, t4178: f64, t4261: f64, t5593: f64, t843: f64, t849: f64, t9559: f64, t9642: f64) -> (f64, f64) {
    let t16969 = t13222 * t16968;
    let t16976 = t16673 * t842;
    let t16979 = t2643 * t16914 / 384.0_f64 + t2643 * t16918 / 768.0_f64 + t9642 * t5593 / 384.0_f64 + t2643 * t16924 / 384.0_f64 - t4178 * t16928 / 192.0_f64 + t13345 - t4178 * t16932 / 384.0_f64 + t4178 * t16937 / 768.0_f64 + 7.0_f64 / 4608.0_f64 * t16940 + 7.0_f64 / 4608.0_f64 * t16942 + 5.0_f64 / 384.0_f64 * t843 * t16946 + 5.0_f64 / 768.0_f64 * t843 * t16951 + 7.0_f64 / 1152.0_f64 * t16954 - t9559 * t16957 / 4.0_f64 + t2571 * t16961 / 8.0_f64 + t2571 * t16965 / 16.0_f64 + t2643 * t16969 / 384.0_f64 - t13365 * t1516 / 384.0_f64 - t4172 * t4261 / 384.0_f64 - t16976 * t849 / 768.0_f64;
    (t16969, t16979)
}
