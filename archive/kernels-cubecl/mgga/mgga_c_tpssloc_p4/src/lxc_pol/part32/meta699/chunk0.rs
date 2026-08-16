//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2185/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2185<F: Float>(t225: F, t28108: F, t22674: F, t28232: F, t6897: F, t28195: F, t6883: F, t22633: F, t22635: F, t26337: F, t5353: F, t5325: F, t90488: F) -> (F, F, F, F, F) {
    let t97558 = t28108 * t225;
    let t97571 = t6897 * t22674 * t28232;
    let t97573 = t6883 * t28195;
    let t97577 = t22633 * t22635 * t26337 * t5353;
    let t97583 = t22633 * t22635 * t90488 * t5325;
    (t97558, t97571, t97573, t97577, t97583)
}
