//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1045/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1045<F: Float>(t113: F, t29222: F, t3090: F, t481: F, t9235: F, t2526: F, t2841: F, t3216: F, t494: F, t2530: F, t7338: F, t3016: F) -> (F, F, F, F, F, F, F) {
    let t29467 = t29222 * t113;
    let t29471 = t3090 * t481;
    let t29496 = t9235 * t481;
    let t29500 = t2841 * t2526;
    let t29699 = t3216 * t494;
    let t29700 = t29699 * t113;
    let t29726 = t7338 * t2530;
    let t29730 = t3016 * t494;
    (t29467, t29471, t29496, t29500, t29700, t29726, t29730)
}
