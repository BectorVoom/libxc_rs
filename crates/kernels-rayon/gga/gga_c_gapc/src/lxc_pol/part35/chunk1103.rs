//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1103/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1103(t11742: f64, t129: f64, t15805: f64, t11741: f64, t28370: f64, t7200: f64, t19055: f64, t3284: f64, t11938: f64, t325: f64, t9386: f64, t1044: f64, t654: f64) -> (f64, f64, f64, f64, f64) {
    let t33701 = t15805 * t129 * t11742;
    let t33704 = t11741 * t28370 * t7200;
    let t33707 = t11741 * t3284 * t19055;
    let t33710 = t325 * t9386 * t11938;
    let t33712 = t654 * t1044;
    (t33701, t33704, t33707, t33710, t33712)
}
