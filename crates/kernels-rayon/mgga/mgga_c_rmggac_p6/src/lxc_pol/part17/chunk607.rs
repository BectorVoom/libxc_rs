//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 607/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk607(t1635: f64, t645: f64, t4044: f64, t2060: f64, t5898: f64, t903: f64, t1614: f64, t649: f64, t27: f64, t2139: f64, t2333: f64, t7508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8548 = t645 * t1635;
    let t8549 = t4044 * t8548;
    let t8551 = t2060 * t5898;
    let t8552 = t903 * t8551;
    let t8561 = t649 * t1614;
    let t8562 = t27 * t8561;
    let t8563 = t2139 * t8562;
    let t8565 = t7508 * t2333;
    (t8548, t8549, t8551, t8552, t8562, t8563, t8565)
}
