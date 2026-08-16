//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 934/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk934(t157: f64, t406: f64, t847: f64, t2248: f64, t469: f64, t103: f64, t2236: f64, t30005: f64, t3054: f64, t633: f64, t865: f64, t2245: f64, t7924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32194 = t847 * t406 * t157;
    let t32262 = t2248 * t469;
    let t32278 = t103 * t2248;
    let t32315 = t30005 * t2236;
    let t32324 = 0.39512695097613069591e1_f64 * t3054 * t633 * t865;
    let t32329 = t7924 * t2245;
    (t32194, t32262, t32278, t32315, t32324, t32329)
}
