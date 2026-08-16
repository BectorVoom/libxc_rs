//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1077/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1077(t1629: f64, t760: f64, t477: f64, t5077: f64, t6636: f64, t332: f64, t5094: f64, t5083: f64, t5084: f64, t12684: f64, t5095: f64, t4103: f64, t872: f64) -> (f64, f64, f64, f64, f64) {
    let t12790 = t760 * t1629;
    let t12794 = 2.0_f64 / 15.0_f64 * t5077 * t6636 * t12790 * t477;
    let t12795 = t12790 * t332;
    let t12798 = 2.0_f64 / 15.0_f64 * t5077 * t5094 * t12795;
    let t12801 = t5083 * t5084 * t12795 / 9.0_f64;
    let t12803 = 4.0_f64 / 15.0_f64 * t12684 * t5095;
    let t12804 = t872 * t4103;
    (t12794, t12798, t12801, t12803, t12804)
}
