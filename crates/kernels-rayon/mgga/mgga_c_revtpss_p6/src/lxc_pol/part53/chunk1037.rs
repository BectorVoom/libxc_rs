//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1037/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1037(t5: f64, t10309: f64, t8736: f64, t136: f64, t7565: f64, t2247: f64, t7574: f64, t8435: f64, t32151: f64, t32586: f64, t32593: f64, t32602: f64, t32795: f64, t8623: f64, t8737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32798 = t10309 * t8736;
    let t32801 = t7565 * t136;
    let t32802 = t2247 * t32801;
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32814 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t32795 * t8623 - 5.0_f64 / 24.0_f64 * t32798 * t32586 - 5.0_f64 / 36.0_f64 * t32802 * t32593 + 5.0_f64 / 144.0_f64 * t32806 * t8623 + 5.0_f64 / 72.0_f64 * t8737 * t32602 + 5.0_f64 / 144.0_f64 * t8737 * t32151);
    (t32798, t32801, t32802, t32805, t32806, t32814)
}
