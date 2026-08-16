//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1472/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472(t17361: f64, t5293: f64, t1261: f64, t12879: f64, t247: f64, t6425: f64, t17416: f64, t5391: f64, t44693: f64, t6421: f64, t1222: f64, t6652: f64, t697: f64) -> (f64, f64, f64, f64, f64) {
    let t69971 = t5293 * t17361;
    let t70032 = t1261 * t247 * t12879 * t6425;
    let t70112 = t5391 * t17416;
    let t70133 = t1261 * t247 * t44693 * t6421;
    let t70225 = t1222 * t697 * t6652;
    (t69971, t70032, t70112, t70133, t70225)
}
