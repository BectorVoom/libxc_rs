//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 530/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk530(t22: f64, t2224: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2226 = 12.0_f64 * t2224 * t22;
    let t2228 = 32.0_f64 * t584 * t588;
    let t2230 = 20.0_f64 * t20 * t27;
    let t2231 = t12 * t19;
    let t2233 = 30.0_f64 * t2231 * t27;
    let t2235 = 72.0_f64 * t592 * t596;
    let t2236 = t21 * t21;
    let t2237 = 1.0_f64 / t2236;
    let t2239 = 42.0_f64 * t25 * t2237;
    (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239)
}
