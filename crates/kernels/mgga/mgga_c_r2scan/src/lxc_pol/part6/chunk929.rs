//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 929/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk929<F: Float>(t6621: F, t6622: F, t1248: F, t806: F, t1218: F, t4912: F, t298: F, t306: F, t307: F, t1257: F, t810: F, t1256: F, t1261: F, t308: F, t6101: F, t1243: F, t1250: F, t1253: F, t295: F, t299: F, t305: F, t6611: F, t803: F, t807: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6623 = t6621 * t6622;
    let t6626 = t1248 * t806;
    let t6627 = t6626 * t1218;
    let t6630 = 3.0 * t4912;
    let t6631 = t298 * t6630;
    let t6635 = 1.0 / t307 / t306;
    let t6636 = t1257 * t810;
    let t6637 = t6635 * t6636;
    let t6640 = t1256 * t810;
    let t6641 = t6640 * t1261;
    let t6644 = -t6630;
    let t6645 = t308 * t6644;
    let t6648 = 154.0 / 27.0 * t6101;
    let t6649 = -440.0 / 27.0 * t6611 * t299 + 200.0 / 9.0 * t1243 * t807 - 50.0 / 9.0 * t803 * t1250 - 25.0 / 3.0 * t803 * t1253 - 10.0 / 27.0 * t295 * t6623 + 10.0 / 3.0 * t295 * t6627 + 5.0 / 3.0 * t295 * t6631 - 10.0 / 27.0 * t305 * t6637 + 10.0 / 3.0 * t305 * t6641 + 5.0 / 3.0 * t305 * t6645 + t6648;
    (t6623, t6627, t6630, t6631, t6635, t6636, t6637, t6641, t6644, t6645, t6648, t6649)
}
