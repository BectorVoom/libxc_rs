//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 897/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk897(t2929: f64, t350: f64, t1482: f64, t947: f64, t3120: f64, t464: f64, t132: f64, t2851: f64, t478: f64, t3055: f64, t432: f64, t1396: f64, t1547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9554 = t350 * t2929;
    let t9577 = t947 * t1482;
    let t9590 = t3120 * t464;
    let t9596 = t132 * t2851 * t478;
    let t9598 = t432 * t3055;
    let t9601 = t132 * t1547 * t1396;
    (t9554, t9577, t9590, t9596, t9598, t9601)
}
