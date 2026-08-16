//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1157/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1157(t10552: f64, t10554: f64, t23096: f64, t23097: f64, t23102: f64, t23103: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t18556: f64) -> (f64, f64) {
    let t23185 = t23096 - t9278 + t9308 + t9316 + t9329 + t9333 + t23097 - t10552 + t10554 + t23102 + t23103;
    let t23186 = 0.54934341918019635162e-3_f64 * t18556;
    (t23185, t23186)
}
