//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1287/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1287<F: Float>(t2148: F, t30285: F, t6165: F, t6407: F, t9423: F, t560: F, t8783: F, t22868: F, t481: F, t26185: F, t113: F, t8701: F, t2147: F, t6086: F, t6398: F, t6535: F, t9296: F) -> (F, F, F, F, F, F, F) {
    let t30287 = t6165 * t2148 * t30285;
    let t30290 = t6407 * t9423;
    let t30292 = t8783 * t560;
    let t30294 = t22868 * t2148 * t30292;
    let t30296 = t8783 * t481;
    let t30298 = t26185 * t2148 * t30296;
    let t30304 = t8701 * t113;
    let t30306 = t2147 * t6086 * t30304;
    let t30312 = t6535 * t6398 * t9296;
    (t30287, t30290, t30294, t30298, t30304, t30306, t30312)
}
