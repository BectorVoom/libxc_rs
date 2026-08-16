//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1102/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1102(t1461: f64, t2911: f64, t2918: f64, t495: f64, t1464: f64, t165: f64, t1832: f64, t8337: f64, t1830: f64, t839: f64, t1447: f64, t5319: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13384 = t1461 * t2911;
    let t13388 = t495 * t2918;
    let t13392 = t165 * t1464;
    let t13399 = t8337 * t1832;
    let t13407 = t1830 * t839;
    let t13432 = t1447 * t5319;
    (t13384, t13388, t13392, t13399, t13407, t13432)
}
