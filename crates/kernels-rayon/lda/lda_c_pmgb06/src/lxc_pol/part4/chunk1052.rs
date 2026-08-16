//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1052/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1052(t1065: f64, t2142: f64, t248: f64, t3890: f64, t897: f64, t2148: f64, t3760: f64, t3705: f64, t26: f64, t5939: f64, t1295: f64, t2236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11174 = t248 * t2142 * t1065;
    let t11177 = t248 * t897 * t3890;
    let t11178 = t2148 * t3760;
    let t11180 = t2148 * t3705;
    let t11200 = t5939 * t26;
    let t11206 = t2236 * t1295;
    (t11174, t11177, t11178, t11180, t11200, t11206)
}
