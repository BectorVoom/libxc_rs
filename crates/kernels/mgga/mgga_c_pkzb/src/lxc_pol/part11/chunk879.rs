//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 879/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk879<F: Float>(t2099: F, t3882: F, t918: F, t3898: F, t6416: F, t8254: F, t2371: F, t3223: F, t1227: F, t2411: F, t300: F, t3061: F, t921: F, t3730: F, t919: F, t2381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10200 = t2099 * t3882;
    let t10201 = t918 * t10200;
    let t10204 = t6416 * t3898;
    let t10205 = t8254 * t10204;
    let t10208 = t2371 * t3223;
    let t10209 = t8254 * t10208;
    let t10212 = t2411 * t1227;
    let t10213 = t300 * t10212;
    let t10214 = t921 * t3061;
    let t10215 = t10213 * t10214;
    let t10220 = t3730 * t919 * t921;
    let t10221 = t2381 * t10220;
    (t10200, t10201, t10204, t10205, t10208, t10209, t10212, t10214, t10215, t10220, t10221)
}
