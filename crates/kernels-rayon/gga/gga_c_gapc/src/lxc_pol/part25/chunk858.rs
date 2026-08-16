//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 858/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk858(t2660: f64, t7880: f64, t8655: f64, t3434: f64, t969: f64, t1936: f64, t928: f64, t943: f64, t3056: f64, t19: f64, t932: f64, t3114: f64) -> (f64, f64, f64, f64, f64) {
    let t9775 = t2660 * t8655 * t7880;
    let t9777 = t3434 * t969;
    let t9779 = t928 * t1936;
    let t9780 = t9779 * t943;
    let t9782 = t928 * t3056;
    let t9783 = t9782 * t943;
    let t9785 = t932 * t19;
    let t9786 = t9785 * t3114;
    (t9775, t9777, t9780, t9783, t9786)
}
