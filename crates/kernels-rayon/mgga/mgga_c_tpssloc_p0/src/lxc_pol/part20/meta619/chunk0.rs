//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2231/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231(t13133: f64, t2655: f64, t13123: f64, t9885: f64, t40738: f64, t10140: f64, t10143: f64, t12971: f64, t1484: f64, t1530: f64, t1877: f64, t2522: f64, t2523: f64, t2749: f64, t39483: f64, t40741: f64, t40743: f64, t40772: f64, t40785: f64, t4255: f64, t4303: f64, t4314: f64, t9470: f64) -> (f64, f64, f64, f64) {
    let t46269 = 12.0_f64 * t13133 * t2655;
    let t46278 = t13123 * t9885;
    let t46279 = 0.16265371950452609763e-1_f64 * t46278;
    let t46280 = 0.65061487801810439052e-1_f64 * t40738;
    let t46281 = -6.0_f64 * t10140 * t1530 * t1877 * t40772 + 6.0_f64 * t10143 * t1877 * t2749 * t4303 + 9.0_f64 * t12971 * t2522 * t2523 + 6.0_f64 * t1484 * t2522 * t40785 - 18.0_f64 * t4255 * t4314 * t9470 + t39483 - t40741 - t40743 + t46269 + t46279 - t46280;
    (t46269, t46279, t46280, t46281)
}
