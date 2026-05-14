//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1275/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1275<F: Float>(t2086: F, t3179: F, t2201: F, t2837: F, t8266: F, t5100: F, t9377: F, t1620: F, t9322: F, t481: F, t9272: F, t6243: F, t1604: F, t20575: F, t9373: F, t113: F, t28325: F) -> (F, F, F, F, F, F, F, F) {
    let t29753 = t3179 * t2086;
    let t29756 = t2201 * t2837 * t8266;
    let t29760 = t5100 * t9377;
    let t29762 = t1620 * t9322;
    let t29764 = t9272 * t481;
    let t29765 = t6243 * t29764;
    let t29766 = t1604 * t29765;
    let t29768 = t20575 * t9373;
    let t29775 = t28325 * t113;
    (t29753, t29756, t29760, t29762, t29765, t29766, t29768, t29775)
}
