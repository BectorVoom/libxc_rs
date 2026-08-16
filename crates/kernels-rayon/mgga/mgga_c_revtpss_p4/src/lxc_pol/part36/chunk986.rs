//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 986/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk986(t2723: f64, t6016: f64, t1558: f64, t5977: f64, t10871: f64, t231: f64, t10552: f64, t10554: f64, t23096: f64, t23097: f64, t23102: f64, t23103: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23160 = t2723 * t6016;
    let t23167 = t5977 * t1558;
    let t23168 = t23167 * t10871;
    let t23172 = t23167 * t2723;
    let t23177 = t23167 * t231;
    let t23185 = t23096 - t9278 + t9308 + t9316 + t9329 + t9333 + t23097 - t10552 + t10554 + t23102 + t23103;
    (t23160, t23167, t23168, t23172, t23177, t23185)
}
