//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1075/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1075(t132: f64, t1547: f64, t2107: f64, t10247: f64, t153: f64, t10203: f64, t435: f64, t5119: f64, t1447: f64, t5282: f64, t1680: f64, t2022: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12112 = t132 * t1547 * t2107;
    let t12139 = t10247 * t153;
    let t12154 = t10203 * t153;
    let t12191 = t132 * t435 * t5119;
    let t12202 = t1447 * t5282;
    let t12224 = t2022 * t1680;
    (t12112, t12139, t12154, t12191, t12202, t12224)
}
