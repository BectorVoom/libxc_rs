//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1941/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1941(t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64, t2340: f64, t94982: f64, t665: f64, t25826: f64, t2366: f64, t13509: f64, t6998: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    let t101455 = t25823 * t4287;
    let t101457 = t1513 * t2340;
    let t101458 = t94982 * t101457;
    let t101460 = t4287 * t665;
    let t101461 = t25826 * t101460;
    let t101463 = t1513 * t2366;
    let t101464 = t25826 * t101463;
    let t101466 = t6998 * t13509;
    (t101451, t101453, t101455, t101458, t101461, t101464, t101466)
}
