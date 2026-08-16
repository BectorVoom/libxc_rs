//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 399/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk399(t1495: f64, t1497: f64, t1395: f64, t1464: f64, t1360: f64, t1364: f64, t1387: f64, t1391: f64, t1399: f64, t1461: f64, t1492: f64, t507: f64) -> (f64, f64, f64, f64) {
    let t1498 = t1495 * t1497;
    let t1499 = t1395 * t1498;
    let t1500 = t1464 * t1499;
    let t1502 = t1360 * t507 - 0.66725e-1_f64 * t1364 * t1387 + t1391 + 0.16581944444444444444e-2_f64 * t1399 + 0.24872916666666666666e-2_f64 * t1461 - 0.24872916666666666666e-2_f64 * t1492 + 0.16581944444444444444e-2_f64 * t1500;
    (t1498, t1499, t1500, t1502)
}
