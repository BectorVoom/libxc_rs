//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 607/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk607(t198: f64, t3464: f64, t186: f64, t493: f64, t191: f64, t717: f64, t187: f64, t190: f64, t1272: f64, t331: f64, t1244: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3465 = t198 * t3464;
    let t3466 = t186 * t3465;
    let t3468 = 4.0_f64 / 15.0_f64 * t493 * t3466;
    let t3469 = t717 * t191;
    let t3472 = 0.02962962962962963_f64 * t190 * t3469 * t187;
    let t3473 = t331 * t1272;
    let t3476 = 1.0_f64 / t1244 / t43;
    (t3465, t3466, t3468, t3469, t3472, t3473, t3476)
}
