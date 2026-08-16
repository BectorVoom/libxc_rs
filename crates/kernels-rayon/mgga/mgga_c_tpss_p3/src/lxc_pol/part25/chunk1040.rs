//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1040/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1040(t10573: f64, t10578: f64, t10584: f64, t10661: f64, t10678: f64, t10679: f64, t10777: f64, t10803: f64, t14322: f64, t14326: f64, t14330: f64, t14334: f64, t14338: f64, t2147: f64, t2173: f64, t3626: f64, t8171: f64, t8204: f64, t8287: f64) -> (f64, f64) {
    let t14343 = t10578 * t10584 * t10573;
    let t14347 = -5.0_f64 / 384.0_f64 * t2173 * t14322 + t2173 * t14326 / 384.0_f64 - t8171 * t14330 / 4.0_f64 + t2147 * t14334 / 8.0_f64 + t2147 * t14338 / 16.0_f64 - t10661 + t10678 - 119.0_f64 / 6912.0_f64 * t10679 - t3626 * t14343 / 192.0_f64 - t8204 - 119.0_f64 / 13824.0_f64 * t8287 - t10777 - t10803;
    (t14343, t14347)
}
