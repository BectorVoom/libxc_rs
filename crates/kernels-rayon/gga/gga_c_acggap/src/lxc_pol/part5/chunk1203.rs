//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1203/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1203(t1524: f64, t1784: f64, t3573: f64, t3621: f64, t6283: f64, t1140: f64, t6279: f64, t1077: f64, t1083: f64, t1173: f64, t1181: f64, t13344: f64, t1532: f64, t16824: f64, t16826: f64, t16839: f64, t16841: f64, t16847: f64, t16849: f64, t1748: f64, t336: f64, t367: f64) -> (f64, f64) {
    let t21901 = t1524 * t1524;
    let t21906 = t3573 * t1784;
    let t21908 = t3621 * t6283;
    let t21910 = t1140 * t6279;
    let t21919 = 0.34299214494455789578e-2_f64 * t1173 * t1181 * t1532 * t1748 * t1077 + t367 * t336 * t1083 * t21901 / 24.0_f64 + 35.0_f64 / 216.0_f64 * t21906 - 7.0_f64 / 24.0_f64 * t21908 - 7.0_f64 / 72.0_f64 * t21910 + 7.0_f64 / 36.0_f64 * t16824 + 7.0_f64 / 72.0_f64 * t16826 - 7.0_f64 / 36.0_f64 * t16839 - 7.0_f64 / 72.0_f64 * t16841 - 7.0_f64 / 36.0_f64 * t16847 - 7.0_f64 / 72.0_f64 * t16849 - 0.80031500487063509016e-2_f64 * t13344;
    (t21901, t21919)
}
