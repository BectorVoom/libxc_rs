//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1661/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1661(t141: f64, t88083: f64, t930: f64, t88091: f64, t41329: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64) -> (f64, f64, f64) {
    let t88168 = t141 * t930 * t88083;
    let t88171 = t141 * t930 * t88091;
    let t88188 = 8.0_f64 / 9.0_f64 * t77559 - 8.0_f64 / 3.0_f64 * t77561 + 40.0_f64 / 81.0_f64 * t77499 - 16.0_f64 / 27.0_f64 * t63453 + 16.0_f64 / 9.0_f64 * t63459 + t41329 + 8.0_f64 * t88085 - 12.0_f64 * t88089 + 2.0_f64 * t88093 + 8.0_f64 / 3.0_f64 * t88097 - 8.0_f64 / 9.0_f64 * t63464;
    (t88168, t88171, t88188)
}
