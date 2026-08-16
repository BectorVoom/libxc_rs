//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1185/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1185(t1772: f64, t336: f64, t1017: f64, t1150: f64, t1180: f64, t1181: f64, t13181: f64, t13184: f64, t13192: f64, t15995: f64, t20972: f64, t21528: f64, t21530: f64, t21532: f64, t3282: f64, t335: f64, t367: f64, t4483: f64, t4593: f64, t4643: f64, t5606: f64, t6289: f64, t6293: f64, t960: f64, t961: f64) -> f64 {
    let t21537 = t336 * t1772;
    let t21550 = 0.34299214494455789578e-2_f64 * t1180 * t1181 * t15995 * t5606 + 0.34299214494455789578e-2_f64 * t1180 * t1181 * t4643 * t20972 + 0.42874018118069736972e-3_f64 * t13181 + 0.34013387707001991332e-1_f64 * t13184 + 0.12862205435420921092e-2_f64 * t13192 - 0.80031500487063509015e-2_f64 * t21528 + 0.80031500487063509015e-2_f64 * t21530 + t1150 * t960 * t21532 * t1017 / 8.0_f64 + t335 * t21537 * t961 / 24.0_f64 + t1150 * t4593 * t4483 / 8.0_f64 + t335 * t3282 * t6289 / 24.0_f64 + t367 * t3282 * t6293 / 24.0_f64;
    t21550
}
