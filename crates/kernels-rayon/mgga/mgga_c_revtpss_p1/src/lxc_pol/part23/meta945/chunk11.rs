//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3115/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3115(t58452: f64, t68454: f64, t68456: f64, t68538: f64, t68540: f64, t68548: f64, t68550: f64, t68567: f64, t68583: f64, t68585: f64, t68590: f64, t81539: f64) -> f64 {
    let t81995 = 0.5519e-1_f64 * t81539 - 0.66228e0_f64 * t68538 - 0.99342e0_f64 * t68540 + 0.11038e0_f64 * t68548 + 0.33114e0_f64 * t68550 - 0.12077e1_f64 * t68454 - 0.181155e1_f64 * t68456 - 0.16557e0_f64 * t68567 + t58452 + 0.27595e0_f64 * t68583 + 0.5519e0_f64 * t68585 - 0.91983333333333333334e-1_f64 * t68590;
    t81995
}
