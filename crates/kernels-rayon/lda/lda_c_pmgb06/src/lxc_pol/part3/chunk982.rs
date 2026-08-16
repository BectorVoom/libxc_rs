//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 982/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk982(t301: f64, t413: f64, t5567: f64, t642: f64, t794: f64, t113: f64, t346: f64, t384: f64, t5583: f64, t5883: f64, t6006: f64, t8097: f64, t8099: f64, t8105: f64, t8108: f64, t8163: f64, t8177: f64, t8180: f64, t8184: f64, t8185: f64, t8187: f64, t8466: f64, t8470: f64) -> (f64, f64) {
    let t11674 = t5567 * t413 * t301;
    let t11676 = t642 * t794;
    let t11678 = t11676 * t113 * t301;
    let t11684 = -t8097 + t8099 + t8105 - 5.4655730795145296e-05_f64 * t8108 - 9.0_f64 * t5583 * t8470 + 6.0_f64 * t6006 * t8466 + 3.0_f64 * t346 * t5883 * t384 + 0.004067943812504169_f64 * t11674 - 0.006715335817467199_f64 * t11678 - 0.03592270203076383_f64 * t8163 - t8177 - 1.370765728342244e-05_f64 * t8180 - t8184 + 0.019957056683757683_f64 * t8185 + 0.11974234010254609_f64 * t8187;
    (t11676, t11684)
}
