//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 853/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk853(t4398: f64, t4401: f64, t4403: f64, t4406: f64, t4408: f64, t4412: f64, t4416: f64, t4418: f64, t5690: f64, t5695: f64, t2696: f64, t2699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7327 = 0.032530742648344574_f64 * t4398;
    let t7328 = 36.0_f64 * t4401;
    let t7329 = 96.0_f64 * t4403;
    let t7330 = 3.0_f64 * t4406;
    let t7332 = 60.0_f64 * t4408;
    let t7333 = 3.5089340384731225_f64 * t4412;
    let t7334 = 1.898172889849454_f64 * t4416;
    let t7335 = 2.0538164420033334_f64 * t4418;
    let t7350 = 24.0_f64 * t5690;
    let t7353 = 24.0_f64 * t5695;
    let t8097 = 1.8960024086108225_f64 * t2696;
    let t8098 = 0.06506148529668915_f64 * t2699;
    (t7327, t7328, t7329, t7330, t7332, t7333, t7334, t7335, t7350, t7353, t8097, t8098)
}
