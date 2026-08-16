//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 919/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk919(t113: f64, t1798: f64, t247: f64, t301: f64, t1147: f64, t123: f64, t2164: f64, t317: f64, t2257: f64, t26: f64, t329: f64, t413: f64, t5567: f64) -> (f64, f64, f64, f64) {
    let t11628 = t247 * t1798 * t113 * t301;
    let t11629 = 0.004067943812504169_f64 * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = 0.5945049527603057_f64 * t11632;
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11674 = t5567 * t413 * t301;
    (t11629, t11633, t11640, t11674)
}
