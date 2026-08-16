//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1955/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1955(t22271: f64, t27940: f64, t22163: f64, t6871: f64, t94429: f64, t22159: f64, t98115: f64, t22120: f64, t26028: f64, t22076: f64, t22102: f64, t94423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108512 = t27940 * t22271;
    let t108514 = t27940 * t22163;
    let t108516 = t94429 * t6871;
    let t108518 = t98115 * t22159;
    let t108520 = t26028 * t22120;
    let t108522 = t26028 * t22076;
    let t108524 = t94423 * t22102;
    (t108512, t108514, t108516, t108518, t108520, t108522, t108524)
}
