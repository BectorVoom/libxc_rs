//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 361/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk361<F: Float>(t1399: F, t20: F, t639: F, t1392: F, t1395: F, t392: F, t22: F, t263: F, t6: F, t1393: F, t1396: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1400 = F::cast_from(0.40256666666666666667e0_f64) * t1399;
    let t1401 = t639 * t20;
    let t1402 = t1401 * t1392;
    let t1403 = F::cast_from(0.366775e-1_f64) * t1402;
    let t1404 = t392 * t1395;
    let t1405 = F::cast_from(0.73355e-1_f64) * t1404;
    let t1407 = t22 * t6 * t263;
    let t1408 = F::cast_from(0.137975e0_f64) * t1407;
    let t1409 = -F::cast_from(0.57538888888888888889e0_f64) * t1393 + F::cast_from(0.11507777777777777778e1_f64) * t1396 + t1400 + t1403 + t1405 + t1408;
    (t1400, t1401, t1402, t1403, t1404, t1405, t1407, t1408, t1409)
}
