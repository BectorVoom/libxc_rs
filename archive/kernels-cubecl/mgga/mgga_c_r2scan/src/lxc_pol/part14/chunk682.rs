//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 682/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk682<F: Float>(t5125: F, t514: F, t2252: F, t788: F, t2201: F, t785: F, t543: F, t108: F, t110: F, t548: F, t1632: F, t2185: F) -> (F, F, F, F, F) {
    let t5126 = t514 * t5125;
    let t5128 = t788 * t2252;
    let t5130 = t2201 * t785 * t5128;
    let t5132 = t543 * t543;
    let t5134 = t108 / t5132;
    let t5135 = t5134 * t110;
    let t5136 = t5135 * t548;
    let t5142 = t1632 * t2185;
    (t5126, t5130, t5134, t5136, t5142)
}
