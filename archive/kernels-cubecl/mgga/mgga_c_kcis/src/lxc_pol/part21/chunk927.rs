//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 927/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk927<F: Float>(t14085: F, t10243: F, t10255: F, t10257: F, t14051: F, t14065: F, t14070: F, t14075: F, t14079: F, t14081: F, t300: F, t3049: F, t4978: F) -> F {
    let t14086 = F::cast_from(0.22109259259259259258e-2_f64) * t14085;
    let t14089 = -F::cast_from(0.55273148148148148147e-3_f64) * t14065 - F::cast_from(0.73697530864197530861e-3_f64) * t14070 + F::cast_from(0.66327777777777777776e-2_f64) * t14075 + t14051 * t300 - t14079 - F::cast_from(0.58958024691358024689e-2_f64) * t10243 - F::cast_from(0.44218518518518518517e-2_f64) * t14081 - F::cast_from(0.13345e0_f64) * t3049 * t4978 - t14086 - F::cast_from(0.88437037037037037034e-2_f64) * t10255 + F::cast_from(0.1621345679012345679e-1_f64) * t10257;
    t14089
}
