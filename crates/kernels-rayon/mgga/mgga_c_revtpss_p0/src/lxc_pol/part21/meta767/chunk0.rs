//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2719/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2719(t50014: f64, t50033: f64, t162: f64, t187: f64, t40092: f64, t40094: f64, t14365: f64, t14397: f64, t2403: f64, t39818: f64, t39823: f64, t40084: f64, t40088: f64, t49992: f64, t49994: f64, t49995: f64) -> (f64, f64, f64, f64, f64) {
    let t50034 = t50014 + t50033;
    let t50037 = 0.19751673498613801407e-1_f64 * t50034 * t162 * t187;
    let t50038 = 0.15584273195113317383e3_f64 * t40092;
    let t50039 = 0.10526802520742363173e2_f64 * t40094;
    let t50040 = -18.0_f64 * t14365 * t14397 * t2403 - t39818 - t39823 + t40084 + t40088 + t49992 + t49994 - t49995 + t50037 - t50038 + t50039;
    (t50034, t50037, t50038, t50039, t50040)
}
