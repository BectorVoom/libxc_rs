//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1046/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1046<F: Float>(t10666: F, t113: F, t97: F, t13908: F, t795: F, t3270: F, t3347: F, t5086: F, t1064: F, t23040: F, t3348: F, t481: F) -> (F, F, F, F, F) {
    let t37282 = t97 * t10666 * t113;
    let t37285 = t13908 * t795;
    let t37286 = t3270 * t37285;
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    let t37312 = t3348 * t481;
    (t37282, t37286, t37292, t37299, t37312)
}
