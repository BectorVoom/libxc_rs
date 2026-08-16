//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 649/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk649(t6929: f64, t6933: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5884: f64, t5887: f64, t5921: f64, t651: f64, t6765: f64, t6773: f64) -> (f64, f64) {
    let t6934 = t6929 + t6933;
    let t6936 = -t118 * t6765 - 2.0_f64 * t1502 * t1843 - 4.0_f64 * t1519 * t4248 + 2.0_f64 * t1847 * t1911 - t508 * t5877 - 2.0_f64 * t508 * t5884 + t511 * t6934 + t569 * t6773 - 4.0_f64 * t5887 * t651 - 2.0_f64 * t5921 * t651;
    (t6934, t6936)
}
