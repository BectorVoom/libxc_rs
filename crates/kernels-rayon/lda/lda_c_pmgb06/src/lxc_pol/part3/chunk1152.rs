//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1152/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1152(t13752: f64, t1972: f64, t2873: f64, t1447: f64, t5477: f64, t1423: f64, t5268: f64, t13739: f64, t13741: f64, t13743: f64, t13745: f64, t13747: f64, t13749: f64, t13751: f64) -> (f64, f64, f64, f64, f64) {
    let t13753 = 4.0_f64 / 45.0_f64 * t13752;
    let t13755 = 2.0_f64 / 15.0_f64 * t1972 * t2873;
    let t13756 = t1447 * t5477;
    let t13757 = 4.0_f64 / 45.0_f64 * t13756;
    let t13758 = t1423 * t5268;
    let t13759 = 2.0_f64 / 45.0_f64 * t13758;
    let t13760 = t13739 + t13741 + t13743 + t13745 + t13747 + t13749 + t13751 + t13753 + t13755 - t13757 - t13759;
    (t13753, t13755, t13757, t13759, t13760)
}
