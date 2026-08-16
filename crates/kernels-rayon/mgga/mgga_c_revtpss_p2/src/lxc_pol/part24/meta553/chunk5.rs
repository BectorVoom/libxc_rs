//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1649/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649(t41549: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64, t41296: f64, t87145: f64) -> (f64, f64) {
    let t88100 = 0.47488888888888888888e-1_f64 * t77559 - 0.14246666666666666667e0_f64 * t77561 + 0.26382716049382716049e-1_f64 * t77499 - 0.31659259259259259258e-1_f64 * t63453 + 0.94977777777777777776e-1_f64 * t63459 + t41549 + 0.4274e0_f64 * t88085 - 0.6411e0_f64 * t88089 + 0.10685e0_f64 * t88093 + 0.14246666666666666667e0_f64 * t88097 - 0.47488888888888888888e-1_f64 * t63464;
    let t88102 = t41296 * t87145;
    (t88100, t88102)
}
