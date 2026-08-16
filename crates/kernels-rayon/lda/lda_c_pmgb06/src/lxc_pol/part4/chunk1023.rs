//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1023/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1023(t3279: f64, t464: f64, t1450: f64, t1600: f64, t135: f64, t1438: f64, t144: f64, t1461: f64, t1489: f64, t3247: f64, t511: f64, t1464: f64, t164: f64, t170: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10148 = t3279 * t464;
    let t10152 = t1450 * t1600;
    let t10203 = 1.0_f64 / t135 / t1438 * t144;
    let t10216 = t1461 * t1489;
    let t10220 = t3247 * t511;
    let t10230 = 1.0_f64 / t164 / t1464 * t170;
    (t10148, t10152, t10203, t10216, t10220, t10230)
}
