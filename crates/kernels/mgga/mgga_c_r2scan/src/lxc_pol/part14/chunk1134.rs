//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1134/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1134<F: Float>(t27182: F, t3308: F, t6449: F, t10810: F, t1592: F, t8156: F, t25813: F, t6218: F, t24064: F, t5136: F, t10743: F, t2699: F) -> (F, F, F, F, F) {
    let t39759 = t6449 * t3308 * t27182;
    let t39762 = t1592 * t10810 * t8156;
    let t39765 = t6218 * t3308 * t25813;
    let t39768 = t5136 * t3308 * t24064;
    let t39770 = t10743 * t2699;
    (t39759, t39762, t39765, t39768, t39770)
}
