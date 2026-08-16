//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 875/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk875(t1039: f64, t1042: f64, t232: f64, t8595: f64, t3745: f64, t959: f64, t968: f64, t1043: f64, t3666: f64, t247: f64, t254: f64, t257: f64, t285: f64) -> (f64, f64, f64, f64, f64) {
    let t8815 = t1039 * t1039;
    let t8818 = t1042 * t1042;
    let t8822 = 24955.7003795058_f64 * t232 / t8815 * t8595 / t8818;
    let t8824 = t3745 * t959;
    let t8826 = t3745 * t968;
    let t8830 = 578.9512619529313_f64 * t3666 * t8595 * t1043;
    let t8834 = 24.0_f64 * t247 * t254 * t257 * t285;
    (t8822, t8824, t8826, t8830, t8834)
}
