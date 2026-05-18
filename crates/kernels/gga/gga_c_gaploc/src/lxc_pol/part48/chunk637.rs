//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 637/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk637<F: Float>(t10422: F, t10425: F, t10427: F, t10432: F, t10436: F, t10440: F, t10442: F, t10445: F, t10467: F, t9446: F, t9451: F, t11218: F, t569: F) -> (F, F) {
    let t11465 = -F::new(0.63904876589867916126e-1) * t9446 + F::new(0.63904876589867916126e-1) * t9451 - F::new(0.59584149919750711116e-1) * t10422 + F::new(0.59584149919750711116e-1) * t10425 + F::new(0.76685851907841499353e0) * t10427 + F::new(0.76685851907841499353e0) * t10432 - F::new(0.17041300423964777634e0) * t10436 + F::new(0.17041300423964777634e0) * t10440 - F::new(0.76685851907841499353e0) * t10442 - F::new(0.76685851907841499353e0) * t10445 - F::new(0.1022478025437886658e1) * t10467;
    let t11470 = t569 * t11218;
    (t11465, t11470)
}
