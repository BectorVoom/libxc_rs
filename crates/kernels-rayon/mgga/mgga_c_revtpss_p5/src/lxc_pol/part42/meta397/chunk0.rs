//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1345/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1345(t1168: f64, t6487: f64, t1745: f64, t5142: f64, t6506: f64, t6503: f64, t3479: f64, t6502: f64, t5146: f64, t12472: f64, t6486: f64, t1130: f64, t6433: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20606 = t6487 * t1168;
    let t20609 = t1745 * t5142;
    let t20612 = t6506 * t1168;
    let t20615 = t6503 * t1168;
    let t20618 = t6502 * t3479;
    let t20619 = t20618 * t1168;
    let t20622 = t5146 * t5142;
    let t20625 = t6486 * t12472;
    let t20626 = t20625 * t1168;
    let t20629 = t6433 * t1130;
    (t20606, t20609, t20612, t20615, t20619, t20622, t20626, t20629)
}
