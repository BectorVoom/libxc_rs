//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1125/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1125(t1024: f64, t1396: f64, t1402: f64, t1404: f64, t153: f64, t1713: f64, t1734: f64, t1828: f64, t1835: f64, t19510: f64, t20092: f64, t301: f64, t3220: f64, t400: f64, t402: f64, t420: f64, t5060: f64, t5066: f64, t5506: f64, t6045: f64, t6053: f64, t6056: f64, t6061: f64, t6065: f64, t839: f64, t917: f64, t921: f64, t922: f64, t923: f64, t94: f64) -> f64 {
    let t20122 = 60.0_f64 * t1024 * t1402 * t1734 * t922 - 360.0_f64 * t1402 * t1713 * t3220 * t922 - 24.0_f64 * t1402 * t301 * t420 * t5506 - 48.0_f64 * t1396 * t1404 * t94 - 12.0_f64 * t1402 * t6061 * t839 + 3.0_f64 * t153 * t19510 * t402 - 24.0_f64 * t153 * t20092 * t921 - 12.0_f64 * t1828 * t923 + 3.0_f64 * t1835 * t917 + 6.0_f64 * t400 * t6065 + 120.0_f64 * t5060 * t6053 - 48.0_f64 * t5060 * t6056 + 120.0_f64 * t5066 * t6045;
    t20122
}
