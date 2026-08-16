//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2632/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2632(t5154: f64, t9713: f64, t9905: f64, t15968: f64, t67: f64, t758: f64, t17: f64, t750: f64, t2225: f64, t5166: f64, t15921: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54389 = t5154 * t9713;
    let t54392 = t5154 * t9905;
    let t54395 = t15968 * t67 * t758;
    let t54398 = t17 * t15968 * t750;
    let t54400 = t2225 * t5166;
    let t54402 = t592 * t15921;
    (t54389, t54392, t54395, t54398, t54400, t54402)
}
