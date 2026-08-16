//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 687/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk687(t142: f64, t2594: f64, t455: f64, t2610: f64, t2325: f64, t3234: f64, t1558: f64, t2329: f64, t2334: f64, t3243: f64, t1563: f64, t2337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6093 = t142 * t2594;
    let t6094 = t455 * t6093;
    let t6097 = t142 * t2610;
    let t6098 = t455 * t6097;
    let t6101 = t3234 * t2325;
    let t6106 = t1558 * t2329;
    let t6111 = t3243 * t2334;
    let t6116 = t1563 * t2337;
    (t6093, t6094, t6097, t6098, t6101, t6106, t6111, t6116)
}
