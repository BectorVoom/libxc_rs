//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1249/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1249(t106430: f64, t25411: f64, t25431: f64, t27341: f64, t99463: f64, t99466: f64, t2411: f64, t29704: f64, t1032: f64, t6343: f64, t1982: f64, t29807: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t106431 = t25411 * t106430;
    let t106433 = t25431 * t106430;
    let t106446 = t99463 * t27341;
    let t106448 = t99466 * t27341;
    let t106516 = t29704 * t2411;
    let t106655 = t6343 * t1032;
    let t106656 = t1982 * t106655;
    let t106701 = t342 * t29807;
    (t106431, t106433, t106446, t106448, t106516, t106655, t106656, t106701)
}
