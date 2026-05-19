//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 712/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk712<F: Float>(t5152: F, t555: F, t1508: F, t1511: F, t1675: F, t191: F, t1545: F, t546: F, t513: F, t1542: F, t1548: F, t1705: F, t575: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5154 = F::cast_from(0.10254018858216406658e4_f64) * t555 * t5152;
    let t5158 = t1511 * t1508;
    let t5165 = F::new(1.0) / t1675 / t191;
    let t5177 = t1545 * t546;
    let t5178 = F::new(36.0) * t5177;
    let t5179 = t1545 * t513;
    let t5186 = F::new(60.0) * t1542 * t546;
    let t5187 = t1548 * t513;
    let t5189 = t1542 * t513;
    let t5221 = t575 * t1705;
    (t5154, t5158, t5165, t5177, t5178, t5179, t5186, t5187, t5189, t5221)
}
