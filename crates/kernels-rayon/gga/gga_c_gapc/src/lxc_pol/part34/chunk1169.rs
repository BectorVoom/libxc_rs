//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1169/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1169(t11941: f64, t9770: f64, t325: f64, t33643: f64, t11991: f64, t11742: f64, t129: f64, t15805: f64, t11741: f64, t28370: f64, t7200: f64, t19055: f64, t3284: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33694 = t9770 * t11941;
    let t33696 = t325 * t33643;
    let t33697 = t33696 * t11991;
    let t33701 = t15805 * t129 * t11742;
    let t33704 = t11741 * t28370 * t7200;
    let t33707 = t11741 * t3284 * t19055;
    (t33694, t33696, t33697, t33701, t33704, t33707)
}
