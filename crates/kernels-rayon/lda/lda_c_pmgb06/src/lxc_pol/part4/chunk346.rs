//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 346/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk346(t12: f64, t1080: f64, t1083: f64, t1219: f64, t336: f64, t1218: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t1225 = piecewise3(t13, 0.0_f64, -2.0_f64 / 9.0_f64 * t1219 * t1080 + 2.0_f64 / 3.0_f64 * t336 * t1083);
    let t1227 = t1218 / 2.0_f64 + t1225 / 2.0_f64;
    t1227
}
