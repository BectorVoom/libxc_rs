//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 944/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk944(t4619: f64, t464: f64, t1894: f64, t3213: f64, t3055: f64, t802: f64, t1464: f64, t524: f64, t2911: f64, t3357: f64, t13372: f64, t1575: f64, t2918: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13933 = t4619 * t464;
    let t13948 = t3213 * t1894;
    let t13949 = 2.0_f64 / 135.0_f64 * t13948;
    let t14015 = t802 * t3055;
    let t14016 = t14015 / 45.0_f64;
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    let t14127 = 0.03199259259259259_f64 * t13372;
    let t14152 = t1575 * t2918;
    (t13933, t13949, t14016, t14106, t14110, t14127, t14152)
}
