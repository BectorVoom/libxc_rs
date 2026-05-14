//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1047/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1047<F: Float>(t1508: F, t2862: F, t5225: F, t24873: F, t5408: F, t10703: F, t5413: F, t15312: F, t296: F, t31641: F, t4973: F, t6360: F, t2881: F, t1091: F, t29082: F, t15195: F, t7101: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31695 = t2862 * t1508 * t5225;
    let t31698 = t24873 * t5408;
    let t31699 = t10703 * t31698;
    let t31702 = t24873 * t5413;
    let t31703 = t15312 * t31702;
    let t31706 = t296 * t31641;
    let t31709 = t6360 * t4973;
    let t31710 = t2881 * t31709;
    let t31713 = t29082 * t1091;
    let t31714 = t2881 * t31713;
    let t31717 = t15195 * t7101;
    (t31695, t31698, t31699, t31702, t31703, t31706, t31709, t31710, t31713, t31714, t31717)
}
