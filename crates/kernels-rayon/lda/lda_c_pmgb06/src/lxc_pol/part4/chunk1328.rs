//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1328/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1328(t2386: f64, t337: f64, t529: f64, t12529: f64, t12530: f64, t12535: f64, t13300: f64, t17070: f64, t3247: f64, t5065: f64, t6678: f64, t12537: f64, t5139: f64) -> (f64, f64, f64, f64, f64) {
    let t17457 = t2386 * t337 * t529;
    let t17460 = 16.0_f64 / 81.0_f64 * t12529 * t12530 * t17457;
    let t17465 = 64.0_f64 / 81.0_f64 * t5065 * t12535 * t3247 * t13300 * t17070;
    let t17466 = t6678 * t529;
    let t17469 = 8.0_f64 / 27.0_f64 * t12537 * t5139 * t17466;
    (t17457, t17460, t17465, t17466, t17469)
}
