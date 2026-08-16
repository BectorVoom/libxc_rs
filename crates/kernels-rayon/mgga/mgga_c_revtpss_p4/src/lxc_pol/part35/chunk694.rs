//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 694/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk694(t1518: f64, t2089: f64, t2071: f64, t7749: f64, t7391: f64, t7393: f64, t7394: f64, t7396: f64, t7753: f64, t7755: f64, t7757: f64) -> (f64, f64, f64) {
    let t7988 = t2089 * t1518;
    let t7991 = t2071 * t7749;
    let t7997 = -t7391 - t7753 / 24.0_f64 - t7393 + t7394 - 0.85748036236139473944e-3_f64 * t7755 - t7396 - 0.34299214494455789578e-2_f64 * t7757;
    (t7988, t7991, t7997)
}
