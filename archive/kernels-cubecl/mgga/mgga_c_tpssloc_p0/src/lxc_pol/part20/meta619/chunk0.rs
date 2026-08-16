//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2231/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231<F: Float>(t13133: F, t2655: F, t13123: F, t9885: F, t40738: F, t10140: F, t10143: F, t12971: F, t1484: F, t1530: F, t1877: F, t2522: F, t2523: F, t2749: F, t39483: F, t40741: F, t40743: F, t40772: F, t40785: F, t4255: F, t4303: F, t4314: F, t9470: F) -> (F, F, F, F) {
    let t46269 = F::cast_from(12.0_f64) * t13133 * t2655;
    let t46278 = t13123 * t9885;
    let t46279 = F::cast_from(0.16265371950452609763e-1_f64) * t46278;
    let t46280 = F::cast_from(0.65061487801810439052e-1_f64) * t40738;
    let t46281 = -F::cast_from(6.0_f64) * t10140 * t1530 * t1877 * t40772 + F::cast_from(6.0_f64) * t10143 * t1877 * t2749 * t4303 + F::cast_from(9.0_f64) * t12971 * t2522 * t2523 + F::cast_from(6.0_f64) * t1484 * t2522 * t40785 - F::cast_from(18.0_f64) * t4255 * t4314 * t9470 + t39483 - t40741 - t40743 + t46269 + t46279 - t46280;
    (t46269, t46279, t46280, t46281)
}
