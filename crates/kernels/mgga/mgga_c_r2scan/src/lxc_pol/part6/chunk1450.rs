//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1450/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1450<F: Float>(t106: F, t27324: F, t27371: F, t797: F, t97: F, t2271: F, t7098: F, t7101: F, t18996: F, t2266: F, t910: F, t7028: F, t879: F, t1543: F, t2858: F, t2867: F, t795: F) -> (F, F, F, F, F, F) {
    let t27375 = t97 * t106 * (t27324 + t27371) * t797;
    let t27380 = t2271 * t7098;
    let t27382 = t2271 * t7101;
    let t27386 = 3.0 * t2266 * t18996 * t910;
    let t27387 = t879 * t7028;
    let t27393 = 18.0 * t2858 * t2867 * t1543 * t795;
    (t27375, t27380, t27382, t27386, t27387, t27393)
}
