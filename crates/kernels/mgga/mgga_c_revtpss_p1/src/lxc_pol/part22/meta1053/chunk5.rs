//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3724/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3724<F: Float>(t12809: F, t13396: F, t17353: F, t17355: F, t17605: F, t17640: F, t17654: F, t17753: F, t17784: F, t20938: F, t20956: F, t3603: F, t3604: F, t3626: F, t3720: F, t44578: F, t44585: F, t45371: F, t471: F, t5046: F, t56760: F, t56888: F, t57005: F, t57386: F, t6688: F, t70612: F, t70616: F, t70623: F, t70630: F, t70633: F) -> F {
    let t70638 = F::cast_from(0.42874018118069736972e-3_f64) * t12809 * t3720 * t6688 * t56760 + F::cast_from(0.25724410870841842183e-2_f64) * t44578 * t3720 * t6688 * t44585 * t3603 - F::cast_from(0.42874018118069736972e-3_f64) * t45371 * t3720 * t6688 * t44585 * t471 + F::cast_from(0.15244095330869239812e-2_f64) * t17605 * t17640 + F::cast_from(0.3811023832717309953e-3_f64) * t70612 - F::cast_from(0.11433071498151929859e-2_f64) * t56888 * t20938 - F::cast_from(0.57165357490759649296e-3_f64) * t70616 - F::cast_from(0.28582678745379824648e-3_f64) * t57386 + F::cast_from(0.21437009059034868486e-3_f64) * t17753 * t3720 * t20956 * t17784 - F::cast_from(0.57165357490759649296e-3_f64) * t70623 - F::cast_from(0.34299214494455789578e-2_f64) * t57005 * t3626 * t5046 * t13396 - F::cast_from(0.30488190661738479624e-2_f64) * t70630 * t17355 - F::cast_from(0.11433071498151929859e-2_f64) * t17654 * t17353 * t3604 * t70633;
    t70638
}
