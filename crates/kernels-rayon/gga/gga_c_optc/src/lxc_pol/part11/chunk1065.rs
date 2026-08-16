//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1065/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1065(t1422: f64, t8344: f64, t1329: f64, t7668: f64, t7680: f64, t1359: f64, t7501: f64, t1347: f64, t7798: f64, t7341: f64, t7758: f64, t1378: f64, t1781: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30408 = t1422 * t8344;
    let t30661 = t1329 * t7668;
    let t30827 = t1329 * t7680;
    let t31281 = t1359 * t7501;
    let t31288 = t1347 * t7798;
    let t31301 = t1359 * t7341;
    let t31304 = t1347 * t7758;
    let t31479 = t862 * t1781 * t1378;
    (t30408, t30661, t30827, t31281, t31288, t31301, t31304, t31479)
}
