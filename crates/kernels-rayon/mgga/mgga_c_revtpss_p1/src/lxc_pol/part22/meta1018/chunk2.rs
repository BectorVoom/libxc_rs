//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3523/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3523(t1011: f64, t6288: f64, t697: f64, t11710: f64, t19872: f64, t3091: f64, t19979: f64, t3153: f64, t372: f64, t19968: f64, t3111: f64, t15850: f64, t4817: f64) -> (f64, f64, f64, f64, f64) {
    let t66721 = t1011 * t697 * t6288;
    let t66731 = t3091 * t11710 * t19872;
    let t66734 = t372 * t19979 * t3153;
    let t66739 = t19968 * t3111;
    let t66747 = t15850 * t4817;
    (t66721, t66731, t66734, t66739, t66747)
}
