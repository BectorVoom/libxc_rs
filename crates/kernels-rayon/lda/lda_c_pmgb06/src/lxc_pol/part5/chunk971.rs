//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 971/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk971(t1423: f64, t6413: f64, t6416: f64, t6419: f64, t1447: f64, t6399: f64, t6403: f64, t6504: f64, t5499: f64, t6407: f64, t161: f64, t489: f64, t6448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16029 = t1423 * t6413;
    let t16031 = t1423 * t6416;
    let t16033 = t1423 * t6419;
    let t16051 = t1447 * t6399;
    let t16053 = t1447 * t6403;
    let t16055 = t1447 * t6504;
    let t16057 = t5499 * t6407;
    let t16089 = t161 * t489 * t6448;
    (t16029, t16031, t16033, t16051, t16053, t16055, t16057, t16089)
}
