//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 511/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk511(t2549: f64, t522: f64, t519: f64, t1460: f64, t2325: f64, t1459: f64, t2166: f64, t806: f64, t1440: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2550 = t522 * t2549;
    let t2552 = 4.0_f64 / 45.0_f64 * t519 * t2550;
    let t2553 = t1460 * t2325;
    let t2554 = t1459 * t2553;
    let t2556 = 4.0_f64 / 27.0_f64 * t519 * t2554;
    let t2557 = t2166 * t806;
    let t2558 = t1440 * t2557;
    (t2550, t2552, t2553, t2554, t2556, t2557, t2558)
}
