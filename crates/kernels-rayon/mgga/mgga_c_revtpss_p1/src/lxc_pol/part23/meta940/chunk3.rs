//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3090/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090(t1131: f64, t1150: f64, t81403: f64, t81418: f64, t81437: f64, t81472: f64, t81485: f64, t81506: f64, t81538: f64, t81552: f64, t24327: f64, t44012: f64) -> (f64, f64) {
    let t81558 = 1.0_f64 * t1131 * (t81403 + t81418 + t81437 + t81472 + t81485 + t81506 + t81538 + t81552) * t1150;
    let t81560 = 0.51726012919273400301e3_f64 * t44012 * t24327;
    (t81558, t81560)
}
