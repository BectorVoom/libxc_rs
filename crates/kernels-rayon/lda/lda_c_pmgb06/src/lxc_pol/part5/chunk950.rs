//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 950/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk950(t14356: f64, t27: f64, t545: f64, t5632: f64, t187: f64, t3389: f64, t856: f64, t5638: f64, t1409: f64, t1798: f64, t188: f64, t3023: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14357 = 0.6492624817418906_f64 * t14356;
    let t14359 = t5632 * t27 * t545;
    let t14465 = t5632 * t187;
    let t14467 = t856 * t3389;
    let t14471 = t5638 * t187;
    let t14472 = 8.0_f64 * t14471;
    let t14481 = t1798 * t1409 * t188;
    let t14482 = 4.0_f64 * t14481;
    let t14484 = t794 * t3023 * t188;
    (t14357, t14359, t14465, t14467, t14472, t14482, t14484)
}
