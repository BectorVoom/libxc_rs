//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1116/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1116<F: Float>(t14085: F, t10243: F, t10255: F, t10257: F, t14051: F, t14065: F, t14070: F, t14075: F, t14079: F, t14081: F, t300: F, t3049: F, t4978: F) -> F {
    let t14086 = F::new(0.22109259259259259258e-2) * t14085;
    let t14089 = -F::new(0.55273148148148148147e-3) * t14065 - F::new(0.73697530864197530861e-3) * t14070 + F::new(0.66327777777777777776e-2) * t14075 + t14051 * t300 - t14079 - F::new(0.58958024691358024689e-2) * t10243 - F::new(0.44218518518518518517e-2) * t14081 - F::new(0.13345e0) * t3049 * t4978 - t14086 - F::new(0.88437037037037037034e-2) * t10255 + F::new(0.1621345679012345679e-1) * t10257;
    t14089
}
