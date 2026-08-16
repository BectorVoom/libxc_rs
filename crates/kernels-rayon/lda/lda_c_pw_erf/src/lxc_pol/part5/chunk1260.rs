//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1260/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1260(t22596: f64, t4506: f64, t4515: f64, t22584: f64, t22586: f64, t22587: f64, t22588: f64, t22589: f64, t22590: f64, t22591: f64, t22594: f64, t22599: f64, t22602: f64, t22606: f64, t22610: f64) -> (f64, f64) {
    let t22613 = 8.0_f64 / 15.0_f64 * t4506 * t4515 * t22596;
    let t22614 = t22584 + t22586 - t22587 - t22588 - t22589 + t22590 - t22591 - t22594 - t22599 - t22602 - t22606 + t22610 + t22613;
    (t22613, t22614)
}
