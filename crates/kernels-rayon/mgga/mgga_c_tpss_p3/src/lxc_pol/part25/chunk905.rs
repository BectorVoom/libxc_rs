//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 905/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk905(t275: f64, t400: f64, t8662: f64, t235: f64, t3032: f64, t2839: f64, t610: f64, t1039: f64, t2202: f64, t57: f64, t262: f64, t390: f64, t5543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9181 = t275 * t8662 * t400;
    let t9182 = 0.36793333333333333333e0_f64 * t9181;
    let t9185 = t235 * t3032;
    let t9187 = 1.0_f64 / t2839 / t610;
    let t9192 = t2202 * t1039;
    let t9198 = t2839 * t57;
    let t9199 = 1.0_f64 / t9198;
    let t9213 = t262 * t5543 * t390;
    (t9181, t9182, t9185, t9187, t9192, t9199, t9213)
}
