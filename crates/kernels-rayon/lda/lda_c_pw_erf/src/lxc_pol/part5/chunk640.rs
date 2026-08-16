//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 640/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk640(t2030: f64, t3802: f64, t519: f64, t2151: f64, t581: f64, t2176: f64, t529: f64, t1484: f64, t473: f64, t219: f64, t1450: f64, t2171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4834 = t3802 * t2030;
    let t4836 = 16.0_f64 / 135.0_f64 * t519 * t4834;
    let t4841 = t2151 * t581;
    let t4848 = t2176 * t529;
    let t4867 = t473 * t1484;
    let t4868 = t4867 * t219;
    let t4879 = 16.0_f64 / 135.0_f64 * t2171 * t1450;
    (t4834, t4836, t4841, t4848, t4868, t4879)
}
