//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1301/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1301(t98522: f64, t12861: f64, t531: f64, t1650: f64, t4312: f64, t6159: f64, t18171: f64, t28754: f64, t27583: f64, t4455: f64, t613: f64, t2104: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99411 = 0.15476481481481481481e-2_f64 * t98522;
    let t99416 = t12861 * t531;
    let t99419 = t6159 * t99416 * t1650 * t4312;
    let t99422 = t18171 * t28754;
    let t99424 = 0.7722800925925925926e-4_f64 * t27583 * t99422;
    let t99429 = t613 * t4455;
    let t99430 = t4314 * t2104;
    (t99411, t99419, t99422, t99424, t99429, t99430)
}
