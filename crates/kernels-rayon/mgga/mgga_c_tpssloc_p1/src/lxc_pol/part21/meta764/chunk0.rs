//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2640/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2640(t2559: f64, t5194: f64, t5198: f64, t118: f64, t16018: f64, t3739: f64, t794: f64, t16081: f64, t16086: f64, t12214: f64, t67: f64, t792: f64) -> (f64, f64, f64, f64) {
    let t54701 = t2559 * t5194 * t5198;
    let t54705 = t3739 * t118 * t794 * t16018;
    let t54711 = t16081 * t16086;
    let t54718 = t792 * t12214 * t67;
    (t54701, t54705, t54711, t54718)
}
