//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 399/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk399<F: Float>(t1495: F, t1497: F, t1395: F, t1464: F, t1360: F, t1364: F, t1387: F, t1391: F, t1399: F, t1461: F, t1492: F, t507: F) -> (F, F, F, F) {
    let t1498 = t1495 * t1497;
    let t1499 = t1395 * t1498;
    let t1500 = t1464 * t1499;
    let t1502 = t1360 * t507 - F::new(0.66725e-1) * t1364 * t1387 + t1391 + F::cast_from(0.16581944444444444444e-2_f64) * t1399 + F::cast_from(0.24872916666666666666e-2_f64) * t1461 - F::cast_from(0.24872916666666666666e-2_f64) * t1492 + F::cast_from(0.16581944444444444444e-2_f64) * t1500;
    (t1498, t1499, t1500, t1502)
}
