//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 279/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk279<F: Float>(t1395: F, t1498: F, t1464: F, t1360: F, t1364: F, t1387: F, t1391: F, t1399: F, t1461: F, t1492: F, t507: F, t589: F) -> (F, F, F, F) {
    let t1499 = t1395 * t1498;
    let t1500 = t1464 * t1499;
    let t1502 = t1360 * t507 - F::new(0.66725e-1) * t1364 * t1387 + t1391 + F::new(0.16581944444444444444e-2) * t1399 + F::new(0.24872916666666666666e-2) * t1461 - F::new(0.24872916666666666666e-2) * t1492 + F::new(0.16581944444444444444e-2) * t1500;
    let t1503 = t1502 * t589;
    (t1499, t1500, t1502, t1503)
}
