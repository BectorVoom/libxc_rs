//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3242/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3242<F: Float>(t1298: F, t18123: F, t18128: F, t3794: F, t3801: F, t5023: F, t58598: F, t58707: F, t58711: F, t58713: F, t58715: F, t58718: F, t58720: F, t58722: F, t58726: F) -> F {
    let t60155 = -F::cast_from(3.0_f64) * t1298 * t18123 * t3801 * t5023 - F::cast_from(3.0_f64) * t18128 * t3794 * t5023 + t58598 - t58707 - t58711 - t58713 - t58715 + t58718 - t58720 - t58722 - t58726;
    t60155
}
