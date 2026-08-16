//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 912/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk912(t1461: f64, t1489: f64, t3247: f64, t511: f64, t1464: f64, t164: f64, t170: f64, t3259: f64, t458: f64, t1435: f64, t1540: f64, t132: f64, t3442: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10216 = t1461 * t1489;
    let t10220 = t3247 * t511;
    let t10230 = 1.0_f64 / t164 / t1464 * t170;
    let t10247 = t3259 * t458;
    let t10255 = t1435 * t1540;
    let t10267 = t132 * t435 * t3442;
    (t10216, t10220, t10230, t10247, t10255, t10267)
}
