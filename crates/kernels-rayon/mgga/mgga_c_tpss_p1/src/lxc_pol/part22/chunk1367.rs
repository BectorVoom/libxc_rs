//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1367/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1367(t65639: f64, t65643: f64, t65647: f64, t60725: f64, t60731: f64, t60733: f64, t60739: f64, t60744: f64, t60750: f64, t60752: f64, t65636: f64, t65641: f64, t65645: f64) -> f64 {
    let t67183 = 7.0_f64 / 144.0_f64 * t65639;
    let t67185 = 7.0_f64 / 144.0_f64 * t65643;
    let t67187 = 119.0_f64 / 864.0_f64 * t65647;
    let t67191 = -7.0_f64 / 24.0_f64 * t60725 - 35.0_f64 / 54.0_f64 * t60731 + 7.0_f64 / 72.0_f64 * t60733 + t65636 / 192.0_f64 - 7.0_f64 / 144.0_f64 * t60739 - t67183 + t65641 / 192.0_f64 + t67185 - t65645 / 192.0_f64 - t67187 - 35.0_f64 / 288.0_f64 * t60744 - 119.0_f64 / 432.0_f64 * t60750 + 7.0_f64 / 288.0_f64 * t60752;
    t67191
}
