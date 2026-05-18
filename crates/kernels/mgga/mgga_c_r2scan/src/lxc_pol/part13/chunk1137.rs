//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1137/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1137<F: Float>(t2626: F, t503: F, t5119: F, t2842: F, t37699: F, t1577: F, t3308: F, t8034: F, t3295: F, t7524: F, t10760: F, t25670: F, t6093: F) -> (F, F, F, F, F) {
    let t39640 = t503 * t5119 * t2626;
    let t39642 = t37699 * t2842;
    let t39645 = t1577 * t3308 * t8034;
    let t39647 = t3295 * t7524;
    let t39650 = t6093 * t10760 * t25670;
    (t39640, t39642, t39645, t39647, t39650)
}
