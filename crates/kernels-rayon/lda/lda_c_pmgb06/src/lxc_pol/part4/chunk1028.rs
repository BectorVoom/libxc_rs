//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1028/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1028(t199: f64, t3993: f64, t1135: f64, t566: f64, t115: f64, t1194: f64, t4182: f64, t113: f64, t27: f64, t4238: f64, t55: f64, t1183: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t10492 = t3993 * t199;
    let t10494 = t1135 * t566;
    let t10500 = 0.1397792_f64 * t1194 * t4182 * t115;
    let t10505 = 0.00011806781668990758_f64 * t113 * t4238 * t27 * t55 * t115;
    let t10506 = t1183 * t97;
    (t10492, t10494, t10500, t10505, t10506)
}
