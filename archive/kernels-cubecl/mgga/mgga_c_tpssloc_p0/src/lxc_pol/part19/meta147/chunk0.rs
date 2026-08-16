//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 754/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk754<F: Float>(t2632: F, t828: F, t157: F, t2658: F, t228: F, t68: F, t2627: F, t226: F, t814: F, t193: F, t200: F) -> (F, F, F, F, F, F, F, F) {
    let t4182 = t2632 * t828;
    let t4194 = t2658 * t157;
    let t4225 = t228 * t68;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    let t4314 = t193 * t200;
    (t4182, t4194, t4225, t4280, t4281, t4290, t4291, t4314)
}
