//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1258/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1258(t2480: f64, t5211: f64, t2076: f64, t6867: f64, t18308: f64, t18311: f64, t18314: f64, t18317: f64, t18390: f64, t185: f64, t514: f64, t7521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22584 = 4.0_f64 / 5.0_f64 * t5211 * t2480;
    let t22586 = 4.0_f64 / 5.0_f64 * t2076 * t6867;
    let t22587 = 16.0_f64 / 9.0_f64 * t18308;
    let t22588 = 8.0_f64 / 45.0_f64 * t18311;
    let t22589 = 16.0_f64 / 45.0_f64 * t18314;
    let t22590 = 8.0_f64 / 27.0_f64 * t18317;
    let t22591 = 4.0_f64 / 15.0_f64 * t18390;
    let t22593 = t185 * t514 * t7521;
    (t22584, t22586, t22587, t22588, t22589, t22590, t22591, t22593)
}
