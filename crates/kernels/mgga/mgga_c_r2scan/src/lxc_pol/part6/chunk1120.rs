//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1120/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1120<F: Float>(t20242: F, t2155: F, t8088: F, t1414: F, t23: F, t254: F, t255: F, t6077: F, t6311: F, t6321: F, t2080: F, t2086: F, t1632: F, t2184: F, t551: F, t6198: F) -> (F, F, F, F) {
    let t20244 = t2155 * t8088 * t20242;
    let t20253 = 0.20211424382067871469e1 * t254 * t6311 / t23 / t6077 / t1414 * t255 * t6321;
    let t20254 = t2080 * t2086;
    let t20264 = t2184 * t551 * t1632 * t6198;
    (t20244, t20253, t20254, t20264)
}
