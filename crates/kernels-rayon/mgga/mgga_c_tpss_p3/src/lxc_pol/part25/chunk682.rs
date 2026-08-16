//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 682/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk682(t1165: f64, t1338: f64, t2056: f64, t3491: f64, t3493: f64, t3537: f64, t4347: f64, t645: f64, t1170: f64, t1614: f64, t1173: f64, t1288: f64, t3282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4352 = 2.0_f64 * t1165 * t3537 + 2.0_f64 * t1338 * t2056 + 2.0_f64 * t1338 * t4347 + 2.0_f64 * t3493 * t645 + t3491;
    let t4356 = t1170 * t1614;
    let t4357 = 4.0_f64 * t4356;
    let t4358 = t1173 * t1614;
    let t4359 = 4.0_f64 * t4358;
    let t4360 = t3282 * t1288;
    (t4352, t4356, t4357, t4358, t4359, t4360)
}
