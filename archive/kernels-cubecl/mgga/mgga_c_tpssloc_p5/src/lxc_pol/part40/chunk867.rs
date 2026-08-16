//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 867/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk867<F: Float>(t6323: F, t6467: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t4028: F, t510: F, t513: F, t5450: F, t5457: F, t5460: F, t5494: F, t574: F, t6287: F, t6295: F, t652: F) -> (F, F) {
    let t6468 = t6323 + t6467;
    let t6470 = -t113 * t6287 - F::cast_from(2.0_f64) * t1442 * t1774 - F::cast_from(4.0_f64) * t1459 * t4028 + F::cast_from(2.0_f64) * t1778 * t1849 - t510 * t5450 - F::cast_from(2.0_f64) * t510 * t5457 + t513 * t6468 - F::cast_from(4.0_f64) * t5460 * t652 - F::cast_from(2.0_f64) * t5494 * t652 + t574 * t6295;
    (t6468, t6470)
}
