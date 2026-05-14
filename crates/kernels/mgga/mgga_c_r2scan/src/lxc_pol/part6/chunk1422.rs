//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1422/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1422<F: Float>(t22575: F, t1655: F, t22023: F, t22030: F, t22034: F, t22036: F, t22039: F, t22041: F, t22045: F, t22086: F, t22089: F, t23892: F, t26860: F, t2774: F, t5903: F, t595: F, t598: F, t7768: F) -> (F,) {
    let t26862 = 8.0 * t22575;
    let t26863 = -0.675260332e-1 * t595 * t23892 * t598 - 0.2025780996e0 * t7768 * t1655 - 0.2025780996e0 * t2774 * t5903 - t22023 - t22030 + t22034 + t22036 + t22039 - t22041 - t22045 + t22086 + t22089 + 12.0 * t26860 - t26862;
    (t26863,)
}
