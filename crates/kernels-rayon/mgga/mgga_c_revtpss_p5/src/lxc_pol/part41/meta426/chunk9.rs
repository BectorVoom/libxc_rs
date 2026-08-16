//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1495/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1495(t1921: f64, t8372: f64, t31582: f64, t575: f64, t1913: f64, t8389: f64, t117781: f64, t117783: f64, t117789: f64, t117793: f64, t118502: f64, t118533: f64, t118576: f64, t1456: f64, t1458: f64, t2192: f64, t22533: f64, t3: f64, t31329: f64, t31619: f64, t6937: f64, t8302: f64) -> f64 {
    let t118579 = t8372 * t1921;
    let t118583 = t31582 * t575;
    let t118585 = t1913 * t8389;
    let t118587 = t6937 * t8302 + t22533 * t2192 + 2.0_f64 * t31329 * t1921 + t1458 * (t118533 + t118576) + 2.0_f64 * t118579 + t117781 + t3 * t118502 * t575 + t117783 + t117789 + t117793 + t118583 + t1456 * t31619 + 2.0_f64 * t118585;
    t118587
}
