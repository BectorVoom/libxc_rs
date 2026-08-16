//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1100/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1100(t177: f64, t2918: f64, t1531: f64, t1593: f64, t13007: f64, t5091: f64, t12555: f64, t5095: f64, t350: f64, t4862: f64, t4641: f64, t4867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13304 = t177 * t2918;
    let t13308 = t1593 * t1531;
    let t13312 = t13007 * t5091;
    let t13314 = t12555 * t5095;
    let t13332 = t350 * t4862;
    let t13337 = t4641 * t4867;
    (t13304, t13308, t13312, t13314, t13332, t13337)
}
