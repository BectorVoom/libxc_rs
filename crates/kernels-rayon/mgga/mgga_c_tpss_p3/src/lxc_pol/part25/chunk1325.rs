//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1325/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1325(t226: f64, t44960: f64, t14349: f64, t1705: f64, t935: f64, t4578: f64, t750: f64, t4802: f64, t580: f64, t1288: f64, t8096: f64, t19818: f64) -> (f64, f64, f64, f64, f64) {
    let t70134 = t44960 * t226;
    let t70189 = t1705 * t14349 * t935;
    let t70221 = t4578 * t750;
    let t70227 = t580 * t4802;
    let t70236 = t8096 * t1288;
    let t70237 = t70236 * t19818;
    (t70134, t70189, t70221, t70227, t70237)
}
