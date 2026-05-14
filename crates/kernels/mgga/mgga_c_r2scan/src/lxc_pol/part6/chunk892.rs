//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 892/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk892<F: Float>(t495: F, t551: F, t6343: F, t574: F, t5066: F, t529: F, t538: F, t1600: F, t1625: F, t5074: F, t536: F) -> (F, F, F, F, F, F, F) {
    let t6345 = t551 * t6343 * t495;
    let t6346 = t574 * t6345;
    let t6349 = t529 * t538 * t5066;
    let t6352 = t1600 * t1625;
    let t6355 = t529 * t538 * t5074;
    let t6358 = t536 * t536;
    let t6359 = 1.0 / t6358;
    (t6345, t6346, t6349, t6352, t6355, t6358, t6359)
}
