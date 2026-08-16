//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1151/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1151(t103421: f64, t7064: f64, t7997: f64, t822: f64, t2470: f64, t28313: f64, t25387: f64, t2471: f64, t28373: f64, t26519: f64, t99257: f64, t10073: f64, t1579: f64, t2066: f64, t25390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103422 = t7064 * t103421;
    let t103424 = t822 * t7997;
    let t103431 = t28313 * t2470;
    let t103432 = t25387 * t103431;
    let t103449 = t28373 * t2471;
    let t103463 = t99257 * t26519;
    let t103471 = t10073 * t25390 * t2066 * t1579;
    (t103422, t103424, t103431, t103432, t103449, t103463, t103471)
}
