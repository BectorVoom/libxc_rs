//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 838/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk838(t1186: f64, t1770: f64, t4239: f64, t1099: f64, t33: f64, t419: f64, t83: f64, t31: f64, t4001: f64, t122: f64, t302: f64, t1755: f64, t1773: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8081 = 0.0006558687695417436_f64 * t4239 * t1186 * t1770;
    let t8083 = 1.0_f64 / t33 / t1099;
    let t8085 = t8083 * t83 * t419;
    let t8087 = 0.0012955432484775182_f64 * t8085 * t1770;
    let t8088 = t31 * t4001;
    let t8091 = 0.9106331049773876_f64 * t122 * t8088 * t302;
    let t8092 = t1773 * t1755;
    (t8081, t8085, t8087, t8088, t8091, t8092)
}
