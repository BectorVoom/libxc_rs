//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 984/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk984(t1447: f64, t6131: f64, t1989: f64, t5194: f64, t2562: f64, t607: f64, t500: f64, t1423: f64, t6124: f64, t1392: f64, t2592: f64, t2466: f64, t3226: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16920 = t1447 * t6131;
    let t16922 = t5194 * t1989;
    let t16924 = t2562 * t607;
    let t16925 = t16924 * t500;
    let t16927 = t1423 * t6124;
    let t16936 = t2592 * t1392;
    let t16962 = t3226 * t2466;
    (t16920, t16922, t16924, t16925, t16927, t16936, t16962)
}
