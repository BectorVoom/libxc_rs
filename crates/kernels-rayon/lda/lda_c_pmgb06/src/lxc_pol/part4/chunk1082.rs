//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1082/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1082(t835: f64, t9266: f64, t1977: f64, t3223: f64, t11862: f64, t160: f64, t1983: f64, t11903: f64, t5137: f64, t1414: f64, t1639: f64, t27: f64, t34: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12460 = t9266 * t835;
    let t12462 = t3223 * t1977;
    let t12465 = t160 * t11862 * t1983;
    let t12494 = t11903 * t5137;
    let t12497 = t1639 * t1414;
    let t12514 = t27 * t34;
    (t12460, t12462, t12465, t12494, t12497, t12514)
}
