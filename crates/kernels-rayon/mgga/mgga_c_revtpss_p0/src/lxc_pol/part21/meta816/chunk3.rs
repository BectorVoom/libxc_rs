//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2998/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2998(t11991: f64, t4817: f64, t1028: f64, t11792: f64, t15651: f64, t1665: f64, t3208: f64, t3211: f64, t42279: f64, t42902: f64, t42907: f64, t4854: f64, t54687: f64, t54693: f64, t54696: f64, t54699: f64, t54704: f64) -> f64 {
    let t54708 = t11991 * t4817;
    let t54712 = -0.63517063878621832551e-4_f64 * t54687 + 0.34299214494455789577e-2_f64 * t3211 * t15651 - 0.42874018118069736972e-3_f64 * t54693 + 0.12862205435420921092e-2_f64 * t54696 * t3208 + 0.68598428988911579154e-2_f64 * t54699 * t1028 + 0.68598428988911579154e-2_f64 * t11792 * t4854 - 0.85748036236139473944e-3_f64 * t54704 - 0.21437009059034868486e-3_f64 * t42279 * t1665 + 0.28582678745379824648e-3_f64 * t54708 + 0.28582678745379824648e-3_f64 * t42902 - 0.19055119163586549765e-3_f64 * t42907;
    t54712
}
