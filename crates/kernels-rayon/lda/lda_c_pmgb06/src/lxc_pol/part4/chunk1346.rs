//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1346/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1346(t13713: f64, t2470: f64, t3198: f64, t13719: f64, t13721: f64, t13104: f64, t835: f64, t1977: f64, t5305: f64, t1847: f64, t1980: f64, t1983: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17680 = 8.0_f64 / 405.0_f64 * t13713;
    let t17682 = t3198 * t2470 / 27.0_f64;
    let t17683 = 8.0_f64 / 135.0_f64 * t13719;
    let t17684 = 4.0_f64 / 81.0_f64 * t13721;
    let t17686 = 2.0_f64 / 45.0_f64 * t13104 * t835;
    let t17688 = 4.0_f64 / 45.0_f64 * t5305 * t1977;
    let t17691 = 8.0_f64 / 45.0_f64 * t1847 * t1980 * t1983;
    (t17680, t17682, t17683, t17684, t17686, t17688, t17691)
}
