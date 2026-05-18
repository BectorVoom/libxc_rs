//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1031/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1031<F: Float>(t481: F, t7469: F, t2568: F, t3433: F, t2563: F, t1550: F, t7338: F, t2252: F, t921: F, t1543: F, t2841: F, t2567: F) -> (F, F, F, F, F, F, F) {
    let t24454 = t7469 * t481;
    let t24521 = t3433 * t2568;
    let t24573 = t3433 * t2563;
    let t24714 = t7338 * t1550;
    let t24750 = t921 * t2252;
    let t24762 = t2841 * t1543;
    let t24786 = t2567 * t2252;
    (t24454, t24521, t24573, t24714, t24750, t24762, t24786)
}
