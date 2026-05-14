//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1423/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1423<F: Float>(t2670: F, t9477: F, t2667: F, t9452: F, t10085: F, t2177: F, t10117: F, t6518: F, t10130: F, t1632: F, t551: F, t6449: F, t574: F, t9956: F, t10099: F, t2184: F) -> (F, F, F, F, F, F, F) {
    let t34481 = t2670 * t9477;
    let t34483 = t2667 * t9452;
    let t34485 = t2177 * t10085;
    let t34487 = t6518 * t10117;
    let t34492 = t6449 * t551 * t1632 * t10130;
    let t34496 = t574 * t551 * t1632 * t9956;
    let t34500 = t2184 * t551 * t1632 * t10099;
    (t34481, t34483, t34485, t34487, t34492, t34496, t34500)
}
