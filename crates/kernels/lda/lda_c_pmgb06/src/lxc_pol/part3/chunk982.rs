//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 982/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk982<F: Float>(t301: F, t413: F, t5567: F, t642: F, t794: F, t113: F, t346: F, t384: F, t5583: F, t5883: F, t6006: F, t8097: F, t8099: F, t8105: F, t8108: F, t8163: F, t8177: F, t8180: F, t8184: F, t8185: F, t8187: F, t8466: F, t8470: F) -> (F, F) {
    let t11674 = t5567 * t413 * t301;
    let t11676 = t642 * t794;
    let t11678 = t11676 * t113 * t301;
    let t11684 = -t8097 + t8099 + t8105 - F::cast_from(5.4655730795145296e-05_f64) * t8108 - F::new(9.0) * t5583 * t8470 + F::new(6.0) * t6006 * t8466 + F::new(3.0) * t346 * t5883 * t384 + F::cast_from(0.004067943812504169_f64) * t11674 - F::cast_from(0.006715335817467199_f64) * t11678 - F::cast_from(0.03592270203076383_f64) * t8163 - t8177 - F::cast_from(1.370765728342244e-05_f64) * t8180 - t8184 + F::cast_from(0.019957056683757683_f64) * t8185 + F::cast_from(0.11974234010254609_f64) * t8187;
    (t11676, t11684)
}
