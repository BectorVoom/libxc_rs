//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 617/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk617(t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64, t2219: f64, t2221: f64, t2223: f64, t2226: f64, t2228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2230 = 20.0_f64 * t20 * t27;
    let t2231 = t12 * t19;
    let t2233 = 30.0_f64 * t2231 * t27;
    let t2235 = 72.0_f64 * t592 * t596;
    let t2236 = t21 * t21;
    let t2237 = 1.0_f64 / t2236;
    let t2239 = 42.0_f64 * t25 * t2237;
    let t2240 = t2219 - t2221 + t2223 + t2226 - t2228 + t2230 + t2233 - t2235 + t2239;
    (t2230, t2231, t2233, t2236, t2237, t2239, t2240)
}
