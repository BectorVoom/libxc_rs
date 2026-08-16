//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1094/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1094(t460: f64, t5477: f64, t1248: f64, t3302: f64, t471: f64, t5332: f64, t1811: f64, t473: f64, t1214: f64, t1287: f64, t489: f64, t5412: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5478 = t460 * t5477;
    let t5479 = t3302 * t1248;
    let t5480 = t5479 * t471;
    let t5481 = t5332 * t5480;
    let t5486 = t473 * t1811;
    let t5487 = t5486 * t1214;
    let t5491 = t1811 * t1248 * t1287;
    let t5494 = t489 * t5412;
    (t5478, t5480, t5481, t5486, t5487, t5491, t5494)
}
