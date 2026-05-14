//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1387/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1387<F: Float>(t2147: F, t6398: F, t8066: F, t6395: F, t8153: F, t2155: F, t24763: F, t6399: F, t7601: F, t24455: F, t1610: F, t2201: F, t8266: F, t113: F, t24877: F, t2148: F) -> (F, F, F, F, F, F, F) {
    let t26294 = t2147 * t6398 * t8066;
    let t26295 = 0.2037639021386884617e0 * t26294;
    let t26296 = t6395 * t8153;
    let t26297 = 0.2037639021386884617e0 * t26296;
    let t26298 = t2155 * t24763;
    let t26300 = t7601 * t6399;
    let t26301 = 0.2037639021386884617e0 * t26300;
    let t26302 = t2155 * t24455;
    let t26305 = t2201 * t1610 * t8266;
    let t26306 = 0.2037639021386884617e0 * t26305;
    let t26307 = t24877 * t113;
    let t26309 = t2147 * t2148 * t26307;
    (t26295, t26297, t26298, t26301, t26302, t26306, t26309)
}
