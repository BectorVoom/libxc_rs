//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1149/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1149<F: Float>(t22868: F, t27182: F, t3332: F, t10868: F, t6165: F, t8156: F, t22790: F, t25813: F, t8160: F, t26185: F, t26186: F, t1054: F, t5108: F, t7963: F) -> (F, F, F, F, F, F) {
    let t39992 = t22868 * t3332 * t27182;
    let t39995 = t6165 * t10868 * t8156;
    let t39998 = t22790 * t3332 * t25813;
    let t40001 = t6165 * t10868 * t8160;
    let t40004 = t26185 * t3332 * t26186;
    let t40007 = t5108 * t1054 * t7963;
    (t39992, t39995, t39998, t40001, t40004, t40007)
}
