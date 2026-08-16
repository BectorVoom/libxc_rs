//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1928/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1928(t2470: f64, t28313: f64, t25387: f64, t95822: f64, t98892: f64, t95537: f64, t1957: f64, t26550: f64, t25372: f64, t98801: f64, t25386: f64, t2471: f64, t28373: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103431 = t28313 * t2470;
    let t103432 = t25387 * t103431;
    let t103435 = 0.28912093960683998208e-1_f64 * t95822 * t98892;
    let t103437 = 0.51405703062096148812e-1_f64 * t95537 * t98892;
    let t103438 = t1957 * t26550;
    let t103441 = 0.14456046980341999104e-1_f64 * t25372 * t103438 * t98801;
    let t103444 = 0.25702851531048074406e-1_f64 * t25386 * t103438 * t98801;
    let t103449 = t28373 * t2471;
    (t103431, t103432, t103435, t103437, t103441, t103444, t103449)
}
