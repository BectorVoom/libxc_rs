//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 985/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk985(t1447: f64, t6541: f64, t6545: f64, t2470: f64, t3226: f64, t6282: f64, t1969: f64, t5220: f64, t6287: f64, t6528: f64, t2614: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16964 = t1447 * t6541;
    let t16966 = t1447 * t6545;
    let t16968 = t3226 * t2470;
    let t16970 = t1447 * t6282;
    let t16992 = t5220 * t1969;
    let t17004 = t1447 * t6287;
    let t17006 = t1447 * t6528;
    let t17025 = t955 * t2614;
    (t16964, t16966, t16968, t16970, t16992, t17004, t17006, t17025)
}
