//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 748/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk748<F: Float>(t20741: F, t20872: F, t20919: F, t20977: F, t160: F, t20851: F, t1023: F, t1058: F, t149: F, t165: F, t20527: F, t20529: F, t20678: F, t20893: F, t20898: F, t20903: F, t20908: F, t20938: F, t20973: F, t4650: F, t4720: F, t4837: F) -> (F, F, F) {
    let t20979 = t20741 + t20872 + t20919 + t20977;
    let t20981 = t20851 * t160;
    let t20989 = -F::new(3.0) * t1023 * t4837 - F::new(3.0) * t1058 * t4650 - F::new(3.0) * t1058 * t4720 - t149 * t20979 - t165 * t20527 - F::new(2.0) * t165 * t20529 - t165 * t20678 + F::new(12.0) * t20893 - F::new(12.0) * t20898 + F::new(12.0) * t20903 - F::new(6.0) * t20908 - F::new(6.0) * t20938 - F::new(2.0) * t20973 + F::new(2.0) * t20981;
    (t20979, t20981, t20989)
}
