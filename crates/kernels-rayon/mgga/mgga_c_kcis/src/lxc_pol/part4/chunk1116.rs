//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1116/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1116(t14085: f64, t10243: f64, t10255: f64, t10257: f64, t14051: f64, t14065: f64, t14070: f64, t14075: f64, t14079: f64, t14081: f64, t300: f64, t3049: f64, t4978: f64) -> f64 {
    let t14086 = 0.22109259259259259258e-2_f64 * t14085;
    let t14089 = -0.55273148148148148147e-3_f64 * t14065 - 0.73697530864197530861e-3_f64 * t14070 + 0.66327777777777777776e-2_f64 * t14075 + t14051 * t300 - t14079 - 0.58958024691358024689e-2_f64 * t10243 - 0.44218518518518518517e-2_f64 * t14081 - 0.13345e0_f64 * t3049 * t4978 - t14086 - 0.88437037037037037034e-2_f64 * t10255 + 0.1621345679012345679e-1_f64 * t10257;
    t14089
}
