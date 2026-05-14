//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1286/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1286<F: Float>(t2139: F, t2294: F, t8799: F, t29706: F, t6462: F, t3216: F, t549: F, t551: F, t6343: F, t2727: F, t7245: F, t560: F, t8825: F, t2148: F, t7628: F, t481: F) -> (F, F, F, F, F, F) {
    let t30252 = t2139 * t2294 * t8799;
    let t30254 = t6462 * t29706;
    let t30258 = t549 * t551 * t6343 * t3216;
    let t30260 = t7245 * t2727;
    let t30281 = t8825 * t560;
    let t30283 = t7628 * t2148 * t30281;
    let t30285 = t8825 * t481;
    (t30252, t30254, t30258, t30260, t30283, t30285)
}
