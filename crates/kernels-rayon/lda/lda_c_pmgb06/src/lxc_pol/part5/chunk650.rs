//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 650/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk650(t113: f64, t301: f64, t5575: f64, t2174: f64, t413: f64, t26: f64, t789: f64, t329: f64) -> (f64, f64, f64, f64) {
    let t5578 = 0.0005811348303577384_f64 * t5575 * t113 * t301;
    let t5580 = t2174 * t413 * t301;
    let t5582 = t26 * t789;
    let t5583 = t329 * t5582;
    (t5578, t5580, t5582, t5583)
}
