//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1162/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1162(t2100: f64, t2407: f64, t1284: f64, t7838: f64, t39: f64, t8327: f64, t186: f64, t220: f64, t548: f64, t1982: f64, t2499: f64, t2505: f64, t6580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21294 = 4.0_f64 / 5.0_f64 * t2407 * t2100;
    let t21296 = 4.0_f64 / 15.0_f64 * t1284 * t7838;
    let t21299 = -6.0_f64 * t39 - 12.0_f64 * t8327;
    let t21303 = 4.0_f64 / 15.0_f64 * t548 * t186 * t220 * t21299;
    let t21305 = 2.0_f64 / 5.0_f64 * t1982 * t2499;
    let t21307 = 4.0_f64 / 5.0_f64 * t6580 * t2505;
    (t21294, t21296, t21299, t21303, t21305, t21307)
}
