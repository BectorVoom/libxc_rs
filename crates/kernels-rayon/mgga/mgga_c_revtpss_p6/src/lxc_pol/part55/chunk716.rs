//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 716/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk716(t5: f64, t2247: f64, t7565: f64, t55: f64, t60: f64, t606: f64, t6971: f64, t72: f64, t1927: f64, t2122: f64, t6977: f64, t1923: f64, t2123: f64, t6954: f64, t6960: f64, t6963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7566 = t2247 * t7565;
    let t7571 = t55 * t60;
    let t7574 = -5.0_f64 / 6.0_f64 * t7571 * t606 + t6971;
    let t7575 = t7574 * t72;
    let t7576 = t7575 * t1927;
    let t7579 = t2122 * t6977;
    let t7583 = piecewise3(t8, 0.0_f64, -t6954 * t2123 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t7566 * t6960 + t6963 * t2123 / 3.0_f64 - t1923 * t7576 / 6.0_f64 - t1923 * t7579 / 6.0_f64);
    (t7566, t7571, t7574, t7575, t7576, t7579, t7583)
}
