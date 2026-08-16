//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 989/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk989(t5051: f64, t802: f64, t1548: f64, t2592: f64, t1447: f64, t6770: f64, t1887: f64, t2015: f64, t27: f64, t545: f64, t7209: f64, t7179: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17372 = t802 * t5051;
    let t17374 = t2592 * t1548;
    let t17376 = t1447 * t6770;
    let t17506 = t1887 * t2015;
    let t17544 = t7209 * t27 * t545;
    let t17547 = t7179 * t27 * t545;
    (t17372, t17374, t17376, t17506, t17544, t17547)
}
