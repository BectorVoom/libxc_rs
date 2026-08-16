//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 798/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk798(t4391: f64, t4398: f64, t4401: f64, t4403: f64, t4406: f64, t4408: f64, t4412: f64, t6056: f64, t2848: f64, t2850: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7326 = 51.94726769812759_f64 * t4391;
    let t7327 = 0.032530742648344574_f64 * t4398;
    let t7328 = 36.0_f64 * t4401;
    let t7329 = 96.0_f64 * t4403;
    let t7330 = 3.0_f64 * t4406;
    let t7332 = 60.0_f64 * t4408;
    let t7333 = 3.5089340384731225_f64 * t4412;
    let t7336 = 0.0005493466511025948_f64 * t6056;
    let t7337 = t2848 + t2850 + t2852;
    (t7326, t7327, t7328, t7329, t7330, t7332, t7333, t7336, t7337)
}
