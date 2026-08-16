//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 724/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk724(t5: f64, t131: f64, t6687: f64, t178: f64, t1887: f64, t815: f64, t1874: f64, t802: f64, t1: f64, t760: f64, t2381: f64, t332: f64, t395: f64, t5961: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t6688 = t6687 * t131;
    let t6690 = t6688 * t178 / 30.0_f64;
    let t6692 = t1887 * t815 / 15.0_f64;
    let t6694 = t802 * t1874 / 15.0_f64;
    let t6695 = t760 * t1;
    let t6698 = t332 * t2381;
    let t6703 = piecewise3(t6, 0.0_f64, 8.0_f64 * t6695 * t395 + 2.0_f64 * t5 * t5961 + 2.0_f64 * t6698);
    (t6688, t6690, t6692, t6694, t6698, t6703)
}
