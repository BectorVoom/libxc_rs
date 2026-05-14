//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1224/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1224<F: Float>(t26296: F, t6399: F, t7601: F, t1610: F, t2201: F, t8266: F, t10855: F, t25850: F, t489: F, t20791: F, t6161: F, t923: F, t4715: F, t5: F, t966: F, t1398: F, t2804: F) -> (F, F, F, F, F, F, F) {
    let t26297 = 0.2037639021386884617e0 * t26296;
    let t26300 = t7601 * t6399;
    let t26301 = 0.2037639021386884617e0 * t26300;
    let t26305 = t2201 * t1610 * t8266;
    let t26306 = 0.2037639021386884617e0 * t26305;
    let t26319 = t25850 * t10855 * t489;
    let t26327 = t20791 * t923 * t6161;
    let t26356 = t5 * t4715 * t966;
    let t26359 = t5 * t1398 * t2804;
    (t26297, t26301, t26306, t26319, t26327, t26356, t26359)
}
