//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 833/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk833(t8700: f64, t889: f64, t3397: f64, t1068: f64, t2387: f64, t322: f64, t3307: f64, t913: f64, t3288: f64, t7577: f64, t3303: f64, t3300: f64, t7553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9512 = t889 * t8700;
    let t9513 = t9512 * t3397;
    let t9515 = t2387 * t1068;
    let t9516 = t9515 * t322;
    let t9518 = t3307 * t913;
    let t9520 = t3288 * t7577;
    let t9521 = t3303 * t9520;
    let t9523 = t7553 * t3300;
    (t9513, t9516, t9518, t9520, t9521, t9523)
}
