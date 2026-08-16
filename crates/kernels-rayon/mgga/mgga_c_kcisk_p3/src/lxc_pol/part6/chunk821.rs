//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 821/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk821(t1308: f64, t8021: f64, t13485: f64, t8036: f64, t3935: f64, t3973: f64, t8040: f64, t1309: f64, t13917: f64, t8032: f64, t1233: f64, t7922: f64) -> (f64, f64, f64, f64, f64) {
    let t26008 = t8021 * t1308;
    let t26064 = t13485 * t8036;
    let t26065 = t3935 * t26064;
    let t26074 = t3973 * t8040;
    let t26075 = t1309 * t26074;
    let t26085 = t13917 * t8032;
    let t26086 = t1309 * t26085;
    let t26095 = t7922 * t1233;
    (t26008, t26065, t26075, t26086, t26095)
}
