//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1310/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1310(t10285: f64, t27: f64, t2231: f64, t596: f64, t2237: f64, t592: f64, t2236: f64, t3: f64, t25: f64, t2240: f64, t602: f64, t2246: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10287 = 120.0_f64 * t10285 * t27;
    let t10288 = t2231 * t596;
    let t10290 = t592 * t2237;
    let t10292 = t2236 * t3;
    let t10293 = 1.0_f64 / t10292;
    let t10295 = 336.0_f64 * t25 * t10293;
    let t10298 = t2240 * t602;
    let t10301 = t599 * t2246;
    (t10287, t10288, t10290, t10295, t10298, t10301)
}
