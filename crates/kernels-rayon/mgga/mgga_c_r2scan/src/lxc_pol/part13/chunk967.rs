//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 967/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk967(t11013: f64, t2304: f64, t875: f64, t3434: f64, t3439: f64, t106: f64, t1550: f64, t97: f64, t3271: f64, t10918: f64, t3262: f64, t3264: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11014 = 3.0_f64 / 2.0_f64 * t11013;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11018 = 0.1951603679568577289e-3_f64 * t11017;
    let t11020 = t97 * t106 * t1550;
    let t11021 = t11020 * t3271;
    let t11022 = t11021 / 4.0_f64;
    let t11024 = t3262 * t10918 * t3264;
    (t11014, t11015, t11018, t11020, t11022, t11024)
}
