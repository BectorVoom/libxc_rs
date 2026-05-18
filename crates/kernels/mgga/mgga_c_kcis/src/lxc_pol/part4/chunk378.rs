//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 378/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk378<F: Float>(t1305: F, t1309: F, t1320: F, t1319: F, t1410: F, t456: F) -> (F, F, F) {
    let t1414 = F::new(0.41275e-2) * t1305;
    let t1416 = F::new(0.1982e-1) * t1320 - t1414 - F::new(0.41275e-2) * t1309;
    let t1419 = t1410 * t1319 / F::new(4.0) + t456 * t1416 / F::new(2.0);
    (t1414, t1416, t1419)
}
