//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1269/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1269<F: Float>(t6118: F, t8839: F, t3115: F, t3433: F, t20338: F, t3100: F, t19852: F, t20818: F, t28300: F, t113: F, t29222: F, t19877: F, t6086: F, t3090: F, t481: F, t22868: F) -> (F, F, F, F, F, F, F) {
    let t29449 = t6118 * t8839;
    let t29451 = t3433 * t3115;
    let t29452 = t20338 * t29451;
    let t29454 = t3433 * t3100;
    let t29455 = t19852 * t29454;
    let t29457 = t20818 * t28300;
    let t29467 = t29222 * t113;
    let t29469 = t19877 * t6086 * t29467;
    let t29471 = t3090 * t481;
    let t29473 = t22868 * t6086 * t29471;
    (t29449, t29452, t29455, t29457, t29467, t29469, t29473)
}
