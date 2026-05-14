//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 970/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk970<F: Float>(t3270: F, t792: F, t1561: F, t983: F, t2847: F, t498: F, t6343: F, t910: F, t551: F, t566: F, t6512: F, t924: F, t552: F, t7088: F, t133: F, t255: F, t2832: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7206 = t3270 * t792;
    let t7217 = t1561 * t983;
    let t7218 = t7217 * t792;
    let t7221 = t498 * t2847;
    let t7233 = t6343 * t910;
    let t7234 = t551 * t7233;
    let t7235 = t566 * t7234;
    let t7237 = t6512 * t924;
    let t7239 = t552 * t7088;
    let t7240 = t551 * t7239;
    let t7244 = t133 * t2832 * t255;
    (t7206, t7217, t7218, t7221, t7234, t7235, t7237, t7240, t7244)
}
