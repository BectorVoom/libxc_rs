//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1378/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1378<F: Float>(t1551: F, t1554: F, t2122: F, t2124: F, t2133: F, t2139: F, t22756: F, t24276: F, t25283: F, t2557: F, t2562: F, t26135: F, t26141: F, t26147: F, t26148: F, t26151: F, t26153: F, t26155: F, t360: F, t5066: F, t5109: F, t6106: F, t6109: F, t7321: F, t7503: F, t7977: F, t8001: F) -> (F,) {
    let t26169 = 0.16463622957338778996e0 * t2122 * t7321 * t25283 - 0.15602799132097683414e1 * t6106 * t5109 * t24276 + 0.41607464352260489103e1 * t22756 + 0.32927245914677557992e-1 * t26135 + 0.39006997830244208535e0 * t2139 * t360 * t7977 * t1551 - 0.15602799132097683414e1 * t26141 * t6109 - t26147 - 0.76830240467580968651e0 * t26148 + 0.57131963037208741166e-1 * t26151 + 0.19207560116895242163e0 * t26153 - 0.69345773920434148506e0 * t26155 - 0.82318114786693894983e-1 * t2557 * t2124 * t7503 * t1554 + 0.13002332610081402845e0 * t2133 * t360 * t8001 * t1551 + 0.43341108700271342816e-1 * t2133 * t360 * t2562 * t5066;
    (t26169,)
}
