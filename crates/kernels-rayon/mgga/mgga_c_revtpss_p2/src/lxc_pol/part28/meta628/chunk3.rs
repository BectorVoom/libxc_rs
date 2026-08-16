//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2262/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262(t101453: f64, t25823: f64, t4287: f64, t1513: f64, t2340: f64, t94982: f64, t665: f64, t25826: f64, t2366: f64, t13509: f64, t6998: f64, t101448: f64, t101451: f64, t94974: f64, t94979: f64, t94981: f64) -> f64 {
    let t101454 = 4.0_f64 / 3.0_f64 * t101453;
    let t101455 = t25823 * t4287;
    let t101456 = 2.0_f64 / 3.0_f64 * t101455;
    let t101457 = t1513 * t2340;
    let t101458 = t94982 * t101457;
    let t101460 = t4287 * t665;
    let t101461 = t25826 * t101460;
    let t101463 = t1513 * t2366;
    let t101464 = t25826 * t101463;
    let t101466 = t6998 * t13509;
    let t101468 = -t94974 - t101448 - 2.0_f64 / 3.0_f64 * t94979 + t94981 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t101451 - t101454 + t101456 - 3.0_f64 / 4.0_f64 * t101458 + t101461 / 2.0_f64 + t101464 / 4.0_f64 - t101466 / 8.0_f64;
    t101468
}
