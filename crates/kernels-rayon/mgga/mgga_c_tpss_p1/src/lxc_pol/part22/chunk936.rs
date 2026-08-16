//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 936/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk936(t3000: f64, t433: f64, t275: f64, t400: f64, t8662: f64, t2896: f64, t673: f64, t235: f64, t3032: f64, t2839: f64, t610: f64, t1039: f64, t2202: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9176 = 1.0_f64 / t3000 / t433;
    let t9181 = t275 * t8662 * t400;
    let t9182 = 0.36793333333333333333e0_f64 * t9181;
    let t9183 = t673 * t2896;
    let t9185 = t235 * t3032;
    let t9187 = 1.0_f64 / t2839 / t610;
    let t9192 = t2202 * t1039;
    (t9176, t9181, t9182, t9183, t9185, t9187, t9192)
}
