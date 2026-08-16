//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2921/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2921(t23495: f64, t698: f64, t52011: f64, t52018: f64, t77513: f64, t41361: f64, t51974: f64, t51978: f64, t63320: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64) -> (f64, f64, f64) {
    let t77736 = t698 * t23495;
    let t77739 = t52011 * t52018 * t77513;
    let t77747 = 0.36231e1_f64 * t77515 - 0.10064166666666666667e1_f64 * t77518 - 0.543465e1_f64 * t77521 + 0.33114e0_f64 * t77736 - 0.149013e1_f64 * t77739 - t51974 + 0.93932222222222222225e0_f64 * t51978 + 0.16557e0_f64 * t63320 + 0.31310740740740740741e0_f64 * t41361 - 0.60384999999999999999e0_f64 * t77527 - 0.60384999999999999999e0_f64 * t77531 + 0.72462e1_f64 * t77535;
    (t77736, t77739, t77747)
}
