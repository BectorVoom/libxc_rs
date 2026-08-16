//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1229/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1229(t7284: f64, t94600: f64, t7243: f64, t9292: f64, t2022: f64, t9646: f64, t9648: f64, t25875: f64, t94394: f64, t46361: f64, t545: f64, t1032: f64, t9656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94602 = 0.22487184191643109717e-1_f64 * t7284 * t94600;
    let t94608 = 0.17073386770573548589e-1_f64 * t9292 * t7243;
    let t94648 = 0.19637199382202157274e-3_f64 * t9646 * t2022 * t9648;
    let t94649 = t25875 * t94394;
    let t94656 = t46361 * t545;
    let t94667 = t1032 * t9656;
    (t94602, t94608, t94648, t94649, t94656, t94667)
}
