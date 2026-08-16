//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 429/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk429(t301: f64, t579: f64, t336: f64, t2046: f64, t372: f64, t599: f64, t578: f64, t137: f64, t429: f64, t128: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2047 = t579 * t301;
    let t2048 = t336 * t2047;
    let t2049 = t2046 * t2048;
    let t2051 = t599 * t372;
    let t2052 = t336 * t2051;
    let t2053 = t578 * t2052;
    let t2056 = t336 * t429 * t137;
    let t2057 = t578 * t2056;
    let t2059 = t577 * t128;
    (t2048, t2049, t2052, t2053, t2056, t2057, t2059)
}
