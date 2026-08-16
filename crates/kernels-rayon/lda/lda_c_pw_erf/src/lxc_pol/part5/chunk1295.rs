//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1295/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1295(t100: f64, t2594: f64, t163: f64, t169: f64, t299: f64, t7851: f64, t11621: f64, t11626: f64, t11627: f64, t15481: f64, t15484: f64, t15486: f64, t9178: f64, t9180: f64, t9181: f64, t9192: f64, t9195: f64, t9203: f64, t9206: f64, t9211: f64) -> (f64, f64) {
    let t23124 = t2594 * t100;
    let t23150 = t169 * t299 * t7851 * t163;
    let t23152 = t9178 - t9180 - 0.00011865309871651405_f64 * t9181 - 0.09451622166942335_f64 * t15481 + 0.09451622166942335_f64 * t15484 + 0.1890324433388467_f64 * t15486 - 0.1890324433388467_f64 * t9192 - t9195 + 0.09451622166942335_f64 * t9203 + t9206 + 0.0878110494085338_f64 * t9211 - t11621 + t11626 + 0.2835486650082701_f64 * t11627 + 0.008980675507690957_f64 * t23150;
    (t23124, t23152)
}
