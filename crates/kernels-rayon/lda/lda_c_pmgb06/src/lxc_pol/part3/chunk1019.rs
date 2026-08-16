//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1019/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1019(t1902: f64, t3177: f64, t1420: f64, t5254: f64, t5257: f64, t5261: f64, t12106: f64, t12108: f64, t12110: f64, t12113: f64, t12114: f64, t12115: f64, t12116: f64, t12117: f64) -> (f64, f64, f64, f64, f64) {
    let t12119 = t3177 * t1902 / 9.0_f64;
    let t12121 = 2.0_f64 / 9.0_f64 * t1420 * t5254;
    let t12123 = t1420 * t5257 / 9.0_f64;
    let t12125 = 8.0_f64 / 27.0_f64 * t1420 * t5261;
    let t12126 = t12106 + t12108 - t12110 + t12113 - t12114 - t12115 - t12116 - t12117 + t12119 + t12121 + t12123 + t12125;
    (t12119, t12121, t12123, t12125, t12126)
}
