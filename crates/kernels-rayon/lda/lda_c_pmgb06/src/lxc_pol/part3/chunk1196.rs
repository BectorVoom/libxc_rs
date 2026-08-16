//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1196/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1196(t12113: f64, t12114: f64, t12115: f64, t12116: f64, t12117: f64, t12119: f64, t12121: f64, t12123: f64, t12125: f64, t12129: f64, t12131: f64, t12135: f64, t12138: f64, t12142: f64, t12145: f64, t12149: f64, t12153: f64, t12159: f64, t12164: f64, t12168: f64, t12170: f64, t12174: f64, t12179: f64) -> (f64, f64) {
    let t14342 = t12113 - t12114 - t12115 - t12116 - t12117 + t12119 + t12121 + t12123 + t12125 - t12129 + t12131;
    let t14343 = t12135 + t12138 + t12142 + t12145 + t12149 + t12153 + t12159 + t12164 - t12168 - t12170 - t12174 - t12179;
    (t14342, t14343)
}
