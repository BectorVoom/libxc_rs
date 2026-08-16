//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1037/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1037(t11937: f64, t11938: f64, t3439: f64, t772: f64, t3438: f64, t11379: f64, t9894: f64, t829: f64, t9896: f64, t3402: f64, t3708: f64, t9934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11939 = t11937 * t11938;
    let t11941 = t772 * t3439;
    let t11942 = t3438 * t11941;
    let t11944 = t9894 * t11379;
    let t11945 = t829 * t9896;
    let t11946 = t11944 * t11945;
    let t11948 = t3402 * t3708;
    let t11949 = t11948 * t9934;
    (t11939, t11941, t11942, t11944, t11945, t11946, t11948, t11949)
}
