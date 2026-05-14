//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 909/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk909<F: Float>(t1239: F, t3609: F, t1238: F, t413: F, t10471: F, t1281: F, t3662: F, t1278: F, t3668: F, t1280: F, t433: F, t1409: F, t1471: F, t1317: F, t1392: F, t544: F) -> (F, F, F, F, F, F, F, F) {
    let t11178 = t3609 * t1239;
    let t11181 = t1238 * t1238;
    let t11182 = 1.0 / t11181;
    let t11183 = t413 * t11182;
    let t11209 = 0.51588271604938271604e-3 * t10471;
    let t11220 = t3662 * t1281;
    let t11223 = t1278 * t3668;
    let t11228 = t1280 * t1280;
    let t11229 = 1.0 / t11228;
    let t11230 = t433 * t11229;
    let t11322 = t1471 * t1409;
    let t11332 = t1392 * t1317 * t544;
    (t11178, t11183, t11209, t11220, t11223, t11230, t11322, t11332)
}
