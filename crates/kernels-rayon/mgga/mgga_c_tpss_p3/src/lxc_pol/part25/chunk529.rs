//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 529/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk529(t2383: f64, t803: f64, t206: f64, t237: f64, t235: f64, t72: f64, t219: f64, t807: f64, t251: f64, t810: f64, t73: f64, t2157: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2384 = t2383 * t803;
    let t2387 = 1.0_f64 / t237 / t206;
    let t2388 = t235 * t2387;
    let t2389 = t2388 * t72;
    let t2401 = t807 * t219;
    let t2405 = 1.0_f64 / t810 / t251;
    let t2406 = t73 * t2405;
    let t2411 = t2157 * t246;
    (t2384, t2387, t2389, t2401, t2405, t2406, t2411)
}
