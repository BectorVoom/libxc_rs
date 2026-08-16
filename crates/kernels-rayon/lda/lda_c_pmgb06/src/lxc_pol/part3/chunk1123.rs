//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1123/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1123(t10158: f64, t10161: f64, t11991: f64, t1476: f64, t36: f64, t350: f64, t4862: f64, t12864: f64, t506: f64, t4641: f64, t4867: f64, t12563: f64, t2909: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13327 = 2.0_f64 / 27.0_f64 * t10158;
    let t13328 = 2.0_f64 / 27.0_f64 * t10161;
    let t13330 = t36 * t1476 * t11991;
    let t13332 = t350 * t4862;
    let t13335 = t36 * t506 * t12864;
    let t13337 = t4641 * t4867;
    let t13340 = t36 * t2909 * t12563;
    (t13327, t13328, t13330, t13332, t13335, t13337, t13340)
}
