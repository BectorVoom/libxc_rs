//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2998/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2998<F: Float>(t11991: F, t4817: F, t1028: F, t11792: F, t15651: F, t1665: F, t3208: F, t3211: F, t42279: F, t42902: F, t42907: F, t4854: F, t54687: F, t54693: F, t54696: F, t54699: F, t54704: F) -> F {
    let t54708 = t11991 * t4817;
    let t54712 = -F::cast_from(0.63517063878621832551e-4_f64) * t54687 + F::cast_from(0.34299214494455789577e-2_f64) * t3211 * t15651 - F::cast_from(0.42874018118069736972e-3_f64) * t54693 + F::cast_from(0.12862205435420921092e-2_f64) * t54696 * t3208 + F::cast_from(0.68598428988911579154e-2_f64) * t54699 * t1028 + F::cast_from(0.68598428988911579154e-2_f64) * t11792 * t4854 - F::cast_from(0.85748036236139473944e-3_f64) * t54704 - F::cast_from(0.21437009059034868486e-3_f64) * t42279 * t1665 + F::cast_from(0.28582678745379824648e-3_f64) * t54708 + F::cast_from(0.28582678745379824648e-3_f64) * t42902 - F::cast_from(0.19055119163586549765e-3_f64) * t42907;
    t54712
}
