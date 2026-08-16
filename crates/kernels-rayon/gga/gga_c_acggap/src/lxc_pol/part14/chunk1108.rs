//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1108/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1108(t1992: f64, t30692: f64, t7842: f64, t9587: f64, t7839: f64, t9601: f64, t1181: f64, t26757: f64, t599: f64, t7413: f64, t6237: f64, t7561: f64) -> (f64, f64, f64, f64) {
    let t39356 = t30692 * t7842 * t1992 * t9587;
    let t39358 = t7839 * t9601;
    let t39362 = t7413 * t1181 * t599 * t26757;
    let t39364 = t7561 * t6237;
    (t39356, t39358, t39362, t39364)
}
