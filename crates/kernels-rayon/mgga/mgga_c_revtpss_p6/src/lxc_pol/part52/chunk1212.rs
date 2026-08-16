//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1212/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1212(t120108: f64, t120120: f64, t120139: f64, t122004: f64, t122008: f64, t122009: f64, t122010: f64, t122015: f64, t126365: f64, t126376: f64, t32445: f64, t34075: f64) -> f64 {
    let t127847 = 0.25702851531048074406e-1_f64 * t122004 - 0.17135921299530705785e1_f64 * t34075 * t32445 + t120108 - t122008 + t122009 - t122010 - t120120 - t122015 - 0.56468933516960933999e-3_f64 * t126365 + t120139 + 0.37645955677973955999e-4_f64 * t126376;
    t127847
}
