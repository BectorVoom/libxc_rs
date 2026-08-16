//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3740/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3740(t20809: f64, t372: f64, t12772: f64, t21172: f64, t5331: f64, t44307: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> (f64, f64, f64) {
    let t71112 = t372 * t20809;
    let t71117 = t5331 * t12772 * t21172;
    let t71134 = 0.33333333333333333334e-1_f64 * t68253 + 0.37037037037037037037e-2_f64 * t68255 - 0.24691358024691358024e-2_f64 * t68257 + t44307 - 0.61728395061728395061e-2_f64 * t68262 + 0.92592592592592592592e-2_f64 * t68267 + 0.2e0_f64 * t68271 + 0.33333333333333333334e-1_f64 * t68275 - 0.11111111111111111111e-1_f64 * t68277 - 0.11111111111111111111e-1_f64 * t68282 - 0.55555555555555555555e-2_f64 * t68287 - 0.33333333333333333333e-1_f64 * t68292;
    (t71112, t71117, t71134)
}
