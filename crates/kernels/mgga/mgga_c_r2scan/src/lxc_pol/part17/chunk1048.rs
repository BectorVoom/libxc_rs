//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1048/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1048<F: Float>(t560: F, t8825: F, t481: F, t8783: F, t113: F, t8701: F, t2530: F, t921: F, t2182: F, t979: F, t3071: F, t6212: F) -> (F, F, F, F, F, F, F, F) {
    let t30281 = t8825 * t560;
    let t30285 = t8825 * t481;
    let t30292 = t8783 * t560;
    let t30296 = t8783 * t481;
    let t30304 = t8701 * t113;
    let t30320 = t921 * t2530;
    let t30370 = t2182 * t979;
    let t30428 = t6212 * t3071;
    (t30281, t30285, t30292, t30296, t30304, t30320, t30370, t30428)
}
