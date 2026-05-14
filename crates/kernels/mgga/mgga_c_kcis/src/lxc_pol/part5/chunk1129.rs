//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1129/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1129<F: Float>(t21078: F, t5701: F, t531: F, t7080: F, t833: F, t12185: F, t12147: F, t7068: F, t1368: F, t1938: F, t5477: F, t16884: F, t16842: F, t16845: F, t21061: F, t21065: F, t21069: F, t21074: F, t5691: F, t5702: F, t5706: F, t5710: F) -> (F,) {
    let t21079 = t5701 * t21078;
    let t21082 = t7080 * t531;
    let t21083 = t21082 * t833;
    let t21084 = t12185 * t21083;
    let t21087 = t12147 * t7068;
    let t21088 = t1368 * t21087;
    let t21097 = t5477 * t1938;
    let t21098 = t16884 * t21097;
    let t21101 = 11.0 / 324.0 * t21061 - t1368 * t21065 / 288.0 - t1368 * t21069 / 288.0 - t1368 * t21074 / 144.0 + t1368 * t21079 / 216.0 + t1368 * t21084 / 144.0 - t21088 / 432.0 + t5691 * t5706 / 54.0 + t5691 * t5710 / 27.0 - 2.0 / 81.0 * t5691 * t5702 + t16842 / 216.0 + t16845 + t1368 * t21098 / 72.0;
    (t21101,)
}
