//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1108/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1108(t30: f64, t33: f64, t189: f64, t6800: f64, t512: f64, t1344: f64, t3874: f64, t5824: f64, t6785: f64, t1348: f64, t3881: f64, t6416: f64, t6792: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t6801 = t6800 * t189;
    let t6802 = t512 * t6801;
    let t6808 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t3874 * t6785 + 2.0_f64 / 3.0_f64 * t1344 * t5824);
    let t6814 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t3881 * t6792 + 2.0_f64 / 3.0_f64 * t1348 * t6416);
    let t6816 = t6808 / 2.0_f64 + t6814 / 2.0_f64;
    (t6801, t6802, t6816)
}
