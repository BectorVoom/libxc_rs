//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 921/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk921(t12235: f64, t175: f64, t3210: f64, t398: f64, t3476: f64, t932: f64, t1017: f64, t1036: f64, t1459: f64, t864: f64, t1004: f64, t3669: f64, t390: f64) -> (f64, f64, f64, f64) {
    let t14091 = 0.77173232612525526552e-2_f64 * t3210 * t398 * t175 * t12235;
    let t14096 = t3476 * t932;
    let t14101 = t1036 * t398 * t1459 * t1017 * t864;
    let t14105 = 0.68026775414003982664e-1_f64 * t1004 * t3669 * t390;
    (t14091, t14096, t14101, t14105)
}
