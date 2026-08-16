//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 962/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk962(t1170: f64, t4430: f64, t1173: f64, t4377: f64, t724: f64, t489: f64, t2215: f64, t4438: f64, t2206: f64, t10039: f64, t3240: f64, t4409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12913 = 8.0_f64 * t1170 * t4430;
    let t12915 = 8.0_f64 * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = 2.0_f64 * t489 * t12916;
    let t12920 = t4438 * t2215;
    let t12922 = t4438 * t2206;
    let t12924 = 4.0_f64 * t10039;
    let t12993 = 7.0_f64 / 72.0_f64 * t3240 * t4409;
    (t12913, t12915, t12918, t12920, t12922, t12924, t12993)
}
