//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1107/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1107(t2002: f64, t3255: f64, t13144: f64, t13149: f64, t13151: f64, t13153: f64, t13156: f64, t13158: f64, t13160: f64, t13162: f64, t13165: f64, t13167: f64, t13170: f64) -> (f64, f64) {
    let t13172 = t2002 * t3255 / 45.0_f64;
    let t13173 = -t13144 + t13149 + t13151 + t13153 + t13156 + t13158 + t13160 + t13162 + t13165 + t13167 + t13170 + t13172;
    (t13172, t13173)
}
