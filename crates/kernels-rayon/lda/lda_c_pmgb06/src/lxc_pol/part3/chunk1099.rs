//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1099/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1099(t9898: f64, t1554: f64, t161: f64, t2089: f64, t132: f64, t2851: f64, t823: f64, t1512: f64, t2015: f64, t432: f64, t5302: f64, t9921: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13085 = 2.0_f64 / 45.0_f64 * t9898;
    let t13087 = t161 * t1554 * t2089;
    let t13088 = t13087 / 45.0_f64;
    let t13090 = t132 * t2851 * t823;
    let t13091 = 4.0_f64 / 405.0_f64 * t13090;
    let t13092 = t1512 * t2015;
    let t13093 = t13092 / 15.0_f64;
    let t13094 = t432 * t5302;
    let t13095 = 2.0_f64 / 15.0_f64 * t13094;
    let t13096 = 4.0_f64 / 135.0_f64 * t9921;
    (t13085, t13088, t13091, t13093, t13095, t13096)
}
