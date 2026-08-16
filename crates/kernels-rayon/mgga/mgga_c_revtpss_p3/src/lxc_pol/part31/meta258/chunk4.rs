//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1145/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1145(t7045: f64, t857: f64, t7024: f64, t7026: f64, t7032: f64, t7035: f64, t7039: f64, t7042: f64) -> f64 {
    let t7046 = t7045 * t857;
    let t7048 = -t7024 - t7026 / 48.0_f64 - t7032 + t7035 - 0.42874018118069736972e-3_f64 * t7039 - t7042 - 0.17149607247227894789e-2_f64 * t7046;
    t7048
}
