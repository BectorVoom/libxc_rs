//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 603/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk603(t10422: f64, t10425: f64, t10427: f64, t10432: f64, t10436: f64, t10440: f64, t10442: f64, t10445: f64, t10467: f64, t9446: f64, t9451: f64, t11218: f64, t569: f64) -> (f64, f64) {
    let t11465 = -0.63904876589867916126e-1_f64 * t9446 + 0.63904876589867916126e-1_f64 * t9451 - 0.59584149919750711116e-1_f64 * t10422 + 0.59584149919750711116e-1_f64 * t10425 + 0.76685851907841499353e0_f64 * t10427 + 0.76685851907841499353e0_f64 * t10432 - 0.17041300423964777634e0_f64 * t10436 + 0.17041300423964777634e0_f64 * t10440 - 0.76685851907841499353e0_f64 * t10442 - 0.76685851907841499353e0_f64 * t10445 - 0.1022478025437886658e1_f64 * t10467;
    let t11470 = t569 * t11218;
    (t11465, t11470)
}
