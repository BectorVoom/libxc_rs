//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1708/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708(t2062: f64, t2453: f64, t2458: f64, t2411: f64, t7427: f64) -> (f64, f64, f64) {
    let t26576 = t2453 * t2062;
    let t26578 = 0.11565819519348392139e-2_f64 * t26576 * t2458;
    let t26585 = t7427 * t2411;
    (t26576, t26578, t26585)
}
