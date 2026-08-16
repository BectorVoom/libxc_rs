//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 976/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk976(t1236: f64, t3643: f64, t1238: f64, t413: f64, t10471: f64, t1278: f64, t3668: f64, t1280: f64, t433: f64, t1409: f64, t1471: f64, t1317: f64, t1392: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11151 = t1236 * t3643;
    let t11181 = t1238 * t1238;
    let t11182 = 1.0_f64 / t11181;
    let t11183 = t413 * t11182;
    let t11209 = 0.51588271604938271604e-3_f64 * t10471;
    let t11223 = t1278 * t3668;
    let t11228 = t1280 * t1280;
    let t11229 = 1.0_f64 / t11228;
    let t11230 = t433 * t11229;
    let t11322 = t1471 * t1409;
    let t11332 = t1392 * t1317 * t544;
    (t11151, t11183, t11209, t11223, t11230, t11322, t11332)
}
