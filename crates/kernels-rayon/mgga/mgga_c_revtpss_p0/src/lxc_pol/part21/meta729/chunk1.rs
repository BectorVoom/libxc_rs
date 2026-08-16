//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2573/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573(t2661: f64, t5675: f64, t9929: f64, t9934: f64, t9775: f64, t9981: f64, t1398: f64, t3992: f64, t4010: f64, t9956: f64, t3938: f64, t47218: f64) -> (f64, f64, f64, f64) {
    let t47318 = t2661 * t9934 * t9929 * t5675;
    let t47320 = t9775 * t9981;
    let t47325 = t2661 * t3992 * t4010 * t1398 * t9956;
    let t47329 = t2661 * t3992 * t47218 * t3938;
    (t47318, t47320, t47325, t47329)
}
