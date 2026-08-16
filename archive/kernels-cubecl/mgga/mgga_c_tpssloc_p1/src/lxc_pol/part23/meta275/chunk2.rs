//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 961/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk961<F: Float>(t5127: F, t6347: F, t1845: F, t6324: F, t5122: F, t6330: F, t12087: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t12461: F, t193: F, t20523: F, t20524: F, t5126: F, t533: F, t9793: F, t9797: F, t9820: F, t9824: F) -> (F, F) {
    let t20681 = t5127 * t6347;
    let t20684 = t6324 * t1845;
    let t20689 = t5122 * t6330;
    let t20692 = F::cast_from(2.0_f64) * t12461 * t193 * t20684 * t533 + F::cast_from(18.0_f64) * t20681 * t5126 + F::cast_from(18.0_f64) * t20689 * t5126 + t12087 - t12094 + t12103 - t12105 - t12109 - t12114 - t20523 + t20524 + t9793 + t9797 - t9820 - t9824;
    (t20684, t20692)
}
