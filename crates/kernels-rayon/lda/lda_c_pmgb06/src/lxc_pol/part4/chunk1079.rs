//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1079/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1079(t5386: f64, t591: f64, t4111: f64, t5391: f64, t138: f64, t4676: f64, t9175: f64, t1869: f64, t8337: f64, t1830: f64, t810: f64, t4641: f64, t4656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12310 = t5386 * t591;
    let t12312 = t5391 * t4111;
    let t12325 = t138 * t9175 * t4676;
    let t12329 = t8337 * t1869;
    let t12337 = t1830 * t810;
    let t12354 = t4641 * t4656;
    (t12310, t12312, t12325, t12329, t12337, t12354)
}
