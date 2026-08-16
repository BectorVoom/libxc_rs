//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 908/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk908(t11610: f64, t981: f64, t11572: f64, t300: f64, t11467: f64, t11506: f64, t11509: f64, t11114: f64, t11118: f64, t11530: f64, t11533: f64, t11547: f64, t11596: f64, t11600: f64, t11604: f64, t11608: f64) -> (f64, f64, f64, f64) {
    let t11612 = 0.5848223622634646207e0_f64 * t981 * t11610;
    let t11614 = 0.19751673498613801407e-1_f64 * t300 * t11572;
    let t11616 = t11506 * t11467 * t11509;
    let t11618 = 0.10254018858216406658e4_f64 * t981 * t11616;
    let t11619 = t11596 - t11600 + t11604 + t11608 - t11612 + t11614 - t11547 - t11618 - t11530 + t11533 - t11114 + t11118;
    (t11612, t11614, t11618, t11619)
}
