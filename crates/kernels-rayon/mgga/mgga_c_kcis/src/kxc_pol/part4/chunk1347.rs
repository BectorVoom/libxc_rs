//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1347/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1347(t17346: f64, t5910: f64, t15860: f64, t5904: f64, t4292: f64, t2061: f64, t4287: f64, t4286: f64, t4266: f64, t6016: f64, t16665: f64, t6028: f64) -> (f64, f64, f64, f64, f64) {
    let t17347 = t17346 * t5910;
    let t17349 = t5904 * t15860;
    let t17350 = t4292 * t17349;
    let t17352 = t2061 * t4287;
    let t17353 = t4286 * t17352;
    let t17355 = t6016 * t4266;
    let t17357 = t6028 * t16665;
    (t17347, t17350, t17353, t17355, t17357)
}
