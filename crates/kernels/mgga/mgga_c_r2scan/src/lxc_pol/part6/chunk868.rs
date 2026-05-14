//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 868/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk868<F: Float>(t2134: F, t481: F, t2148: F, t6165: F, t1554: F, t2140: F, t360: F, t2124: F, t2125: F, t1600: F, t1629: F, t2078: F, t537: F, t255: F, t571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6166 = t2134 * t481;
    let t6167 = t2148 * t6166;
    let t6168 = t6165 * t6167;
    let t6170 = t2140 * t1554;
    let t6171 = t360 * t6170;
    let t6175 = t2124 * t2125 * t1554;
    let t6178 = t1600 * t1629;
    let t6180 = t537 * t2078;
    let t6182 = t571 * t6180 * t255;
    (t6166, t6167, t6168, t6170, t6171, t6175, t6178, t6180, t6182)
}
