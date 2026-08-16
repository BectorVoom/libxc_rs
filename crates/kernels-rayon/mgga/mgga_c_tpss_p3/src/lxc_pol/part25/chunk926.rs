//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 926/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk926(t555: f64, t7622: f64, t123: f64, t1354: f64, t2349: f64, t3645: f64, t725: f64, t1352: f64, t2332: f64, t2206: f64, t3557: f64, t2215: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10350 = 2.0_f64 * t555;
    let t10351 = 6.0_f64 * t7622;
    let t10510 = t1354 * t123;
    let t10511 = t10510 * t2349;
    let t10520 = 2.0_f64 * t3645 * t725;
    let t10521 = t1352 * t2332;
    let t10558 = t3557 * t2206;
    let t10560 = t3557 * t2215;
    (t10350, t10351, t10511, t10520, t10521, t10558, t10560)
}
