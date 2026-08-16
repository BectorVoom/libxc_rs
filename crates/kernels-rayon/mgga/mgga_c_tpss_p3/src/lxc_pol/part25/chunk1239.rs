//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1239/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1239(t1163: f64, t118: f64, t1273: f64, t1339: f64, t1663: f64, t1760: f64, t1834: f64, t18898: f64, t20288: f64, t20294: f64, t20396: f64, t20407: f64, t20640: f64, t20642: f64, t3502: f64, t3538: f64, t3542: f64, t4541: f64, t485: f64, t5706: f64, t5801: f64, t5905: f64, t626: f64, t6309: f64, t6409: f64, t6437: f64) -> f64 {
    let t20646 = -t1163 * t6309 - t118 * t20640 + t1273 * t6409 - 2.0_f64 * t1339 * t18898 - 2.0_f64 * t1339 * t20294 + t1663 * t5905 + 3.0_f64 * t1760 * t20407 - t1760 * t20642 + t1834 * t4541 - t20288 * t485 - 2.0_f64 * t20396 * t626 - 2.0_f64 * t3502 * t5801 - 2.0_f64 * t3538 * t5801 - 2.0_f64 * t3542 * t5801 + t5706 * t6437;
    t20646
}
