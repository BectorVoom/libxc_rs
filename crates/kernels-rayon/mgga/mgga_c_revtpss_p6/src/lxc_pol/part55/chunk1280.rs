//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1280/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1280(t5: f64, t130845: f64, t130895: f64, t117: f64, t127532: f64, t127545: f64, t127547: f64, t127549: f64, t127550: f64, t127556: f64, t127559: f64, t128195: f64, t1310: f64, t33578: f64, t33580: f64, t33583: f64, t34776: f64, t508: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t130897 = piecewise3(t8, 0.0_f64, t130845 + t130895);
    let t130898 = t130897 * t117;
    let t130901 = -t130898 * t508 - t1310 * t34776 + t127532 - t127545 - t127547 - t127549 - t127550 - t127556 + t127559 - t128195 - t33578 - t33580 - t33583;
    (t130898, t130901)
}
