//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1247/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1247(t591: f64, t7975: f64, t13440: f64, t13444: f64, t13447: f64, t13450: f64, t20784: f64, t20786: f64, t20789: f64, t20791: f64, t20792: f64, t20794: f64, t20797: f64) -> f64 {
    let t22018 = t7975 * t591;
    let t22021 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + 2.0_f64 / 9.0_f64 * t22018 + t13444 + t13447 + 0.547_f64 * t13450 + t20797;
    t22021
}
