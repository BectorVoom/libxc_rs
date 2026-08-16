//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1918/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1918(t11249: f64, t13045: f64, t3603: f64, t13032: f64, t3609: f64, t1032: f64, t3552: f64, t1246: f64, t247: f64, t3372: f64, t3634: f64, t1261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13046 = t11249 * t13045;
    let t13053 = t11249 * t3603;
    let t13058 = t13032 * t3609;
    let t13068 = t3552 * t1032;
    let t13069 = t13068 * t1246;
    let t13085 = t247 * t3634 * t3372;
    let t13086 = t1261 * t13085;
    (t13046, t13053, t13058, t13068, t13069, t13085, t13086)
}
