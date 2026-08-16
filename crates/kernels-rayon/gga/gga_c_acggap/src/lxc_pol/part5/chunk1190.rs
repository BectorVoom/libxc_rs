//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1190/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1190(t14047: f64, t6347: f64, t16899: f64, t5928: f64, t14056: f64, t5932: f64, t1165: f64, t12945: f64, t13221: f64, t13226: f64, t13229: f64, t16720: f64, t16724: f64, t16728: f64, t16730: f64, t16739: f64, t1884: f64, t945: f64) -> f64 {
    let t21651 = t14047 * t6347;
    let t21657 = t16899 * t5928;
    let t21659 = t14056 * t5932;
    let t21661 = 0.34013387707001991333e-1_f64 * t13221 - t13226 - t13229 + 0.68598428988911579156e-2_f64 * t16720 + 0.34299214494455789578e-2_f64 * t16724 + 0.68598428988911579156e-2_f64 * t16728 - 0.85748036236139473944e-3_f64 * t16730 + 35.0_f64 / 54.0_f64 * t16739 + 0.68598428988911579156e-2_f64 * t21651 + 0.85748036236139473944e-2_f64 * t12945 * t1165 * t1884 * t945 + 0.34299214494455789578e-1_f64 * t21657 + 0.13719685797782315831e-1_f64 * t21659;
    t21661
}
