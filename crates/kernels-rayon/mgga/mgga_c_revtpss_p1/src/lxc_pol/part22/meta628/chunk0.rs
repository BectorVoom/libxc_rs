//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2544/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2544(t1045: f64, t4579: f64, t15691: f64, t1043: f64, t1592: f64, t3155: f64, t4817: f64, t4834: f64, t11933: f64, t11956: f64, t11967: f64, t11972: f64, t11989: f64, t15700: f64, t15830: f64, t16121: f64, t16226: f64, t1675: f64, t3211: f64, t6273: f64, t6278: f64) -> (f64, f64, f64, f64, f64) {
    let t19992 = t1045 * t4579;
    let t19993 = t15691 * t19992;
    let t19996 = t1592 * t1043;
    let t19997 = t3155 * t19996;
    let t19998 = t15691 * t19997;
    let t20005 = t4834 * t4817;
    let t20012 = -0.57165357490759649296e-3_f64 * t15700 * t19993 + 0.57165357490759649296e-3_f64 * t16226 * t19998 - 0.47637797908966374413e-4_f64 * t11956 + 0.2540682555144873302e-3_f64 * t11967 + t11972 - 0.15244095330869239812e-2_f64 * t15830 * t1675 + 0.19055119163586549765e-3_f64 * t20005 - 0.31758531939310916275e-4_f64 * t11989 - t16121 + 0.22866142996303859718e-2_f64 * t11933 * t6273 + 0.11433071498151929859e-2_f64 * t3211 * t6278;
    (t19992, t19993, t19997, t19998, t20012)
}
