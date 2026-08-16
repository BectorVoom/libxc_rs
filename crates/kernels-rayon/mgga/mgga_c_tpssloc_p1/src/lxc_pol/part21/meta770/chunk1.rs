//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2671/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2671(t12461: f64, t6463: f64, t54312: f64, t54314: f64, t5356: f64, t54316: f64, t1307: f64, t16018: f64, t193: f64, t19631: f64, t20081: f64, t3698: f64, t3701: f64, t3719: f64, t39320: f64, t39324: f64, t39327: f64, t5126: f64, t5127: f64, t5160: f64, t533: f64, t571: f64) -> (f64, f64, f64, f64) {
    let t56136 = t6463 * t12461;
    let t56140 = 48.0_f64 * t54312;
    let t56141 = 96.0_f64 * t54314;
    let t56142 = t5356 * t5356;
    let t56147 = 64.0_f64 * t54316;
    let t56148 = 12.0_f64 * t1307 * t19631 * t5126 * t571 - 2.0_f64 * t193 * t3701 * t533 * t56142 + 12.0_f64 * t16018 * t5126 * t5127 + 6.0_f64 * t20081 * t3719 * t5126 + 2.0_f64 * t3698 * t5160 * t56136 + t39320 - t39324 + t39327 - t56140 + t56141 - t56147;
    (t56140, t56141, t56147, t56148)
}
