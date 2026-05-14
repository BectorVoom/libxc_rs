//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1013/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1013<F: Float>(t10760: F, t25997: F, t6085: F, t2147: F, t25573: F, t26997: F, t11724: F, t19883: F, t11675: F, t19872: F, t11678: F, t6395: F, t10764: F, t26282: F, t10882: F, t11748: F) -> (F, F, F, F, F, F, F, F) {
    let t39700 = t6085 * t10760 * t25997;
    let t39703 = t2147 * t10760 * t25573;
    let t39706 = t6085 * t10760 * t26997;
    let t39708 = t19883 * t11724;
    let t39713 = t19872 * t11675;
    let t39715 = t6395 * t11678;
    let t39717 = t26282 * t10764;
    let t39719 = t11748 * t10882;
    (t39700, t39703, t39706, t39708, t39713, t39715, t39717, t39719)
}
