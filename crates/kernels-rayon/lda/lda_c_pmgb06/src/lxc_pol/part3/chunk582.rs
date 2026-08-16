//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 582/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk582(t12: f64, t155: f64, t3134: f64, t1512: f64, t460: f64, t1083: f64, t337: f64, t2938: f64, t44: f64, t131: f64, t178: f64, t436: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t3136 = t3134 * t155 / 30.0_f64;
    let t3138 = t1512 * t460 / 10.0_f64;
    let t3139 = t337 * t1083;
    let t3144 = piecewise3(t13, 0.0_f64, 2.0_f64 * t12 * t2938 + 6.0_f64 * t3139);
    let t3145 = t3144 * t44;
    let t3146 = t3145 * t131;
    let t3148 = t3146 * t178 / 30.0_f64;
    let t3149 = t1512 * t436;
    (t3136, t3138, t3139, t3145, t3146, t3148, t3149)
}
