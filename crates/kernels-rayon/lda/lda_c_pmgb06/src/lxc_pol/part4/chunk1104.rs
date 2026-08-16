//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1104/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1104(t3247: f64, t842: f64, t1447: f64, t5313: f64, t4585: f64, t4589: f64, t1995: f64, t3226: f64, t146: f64, t4989: f64, t9712: f64, t2060: f64, t819: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13483 = t3247 * t842;
    let t13502 = t1447 * t5313;
    let t13504 = t1447 * t4585;
    let t13507 = t1447 * t4589;
    let t13515 = t3226 * t1995;
    let t13532 = t146 * t9712 * t4989;
    let t13558 = t2060 * t819;
    (t13483, t13502, t13504, t13507, t13515, t13532, t13558)
}
