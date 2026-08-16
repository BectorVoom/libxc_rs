//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 978/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk978(t8249: f64, t8251: f64, t402: f64, t4383: f64, t75: f64, t8255: f64, t1034: f64, t1798: f64, t40: f64, t3153: f64, t748: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11352 = 1.7544670192365612_f64 * t8249;
    let t11353 = 3.5089340384731225_f64 * t8251;
    let t11355 = t4383 * t75 * t402;
    let t11356 = 1.7544670192365612_f64 * t11355;
    let t11357 = 24.0_f64 * t8255;
    let t11359 = t40 * t1798 * t1034;
    let t11360 = 3.0_f64 * t11359;
    let t11362 = t40 * t748 * t3153;
    let t11363 = 0.06506148529668915_f64 * t8267;
    (t11352, t11353, t11356, t11357, t11360, t11362, t11363)
}
