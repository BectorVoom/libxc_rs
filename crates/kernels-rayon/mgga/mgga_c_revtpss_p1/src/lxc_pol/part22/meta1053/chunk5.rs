//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3724/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724(t12809: f64, t13396: f64, t17353: f64, t17355: f64, t17605: f64, t17640: f64, t17654: f64, t17753: f64, t17784: f64, t20938: f64, t20956: f64, t3603: f64, t3604: f64, t3626: f64, t3720: f64, t44578: f64, t44585: f64, t45371: f64, t471: f64, t5046: f64, t56760: f64, t56888: f64, t57005: f64, t57386: f64, t6688: f64, t70612: f64, t70616: f64, t70623: f64, t70630: f64, t70633: f64) -> f64 {
    let t70638 = 0.42874018118069736972e-3_f64 * t12809 * t3720 * t6688 * t56760 + 0.25724410870841842183e-2_f64 * t44578 * t3720 * t6688 * t44585 * t3603 - 0.42874018118069736972e-3_f64 * t45371 * t3720 * t6688 * t44585 * t471 + 0.15244095330869239812e-2_f64 * t17605 * t17640 + 0.3811023832717309953e-3_f64 * t70612 - 0.11433071498151929859e-2_f64 * t56888 * t20938 - 0.57165357490759649296e-3_f64 * t70616 - 0.28582678745379824648e-3_f64 * t57386 + 0.21437009059034868486e-3_f64 * t17753 * t3720 * t20956 * t17784 - 0.57165357490759649296e-3_f64 * t70623 - 0.34299214494455789578e-2_f64 * t57005 * t3626 * t5046 * t13396 - 0.30488190661738479624e-2_f64 * t70630 * t17355 - 0.11433071498151929859e-2_f64 * t17654 * t17353 * t3604 * t70633;
    t70638
}
