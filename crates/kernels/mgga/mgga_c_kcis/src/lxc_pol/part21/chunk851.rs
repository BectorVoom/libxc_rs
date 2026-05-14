//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 851/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk851<F: Float>(t14072: F, t14073: F, t3200: F, t2861: F, t4774: F, t4549: F, t9429: F, t4802: F, t10243: F, t10255: F, t10257: F, t14051: F, t14065: F, t14070: F, t300: F, t3049: F, t4978: F) -> (F, F, F, F, F) {
    let t14074 = t14072 * t14073;
    let t14075 = t3200 * t14074;
    let t14078 = t2861 * t4774;
    let t14079 = 0.33163888888888888888e-2 * t14078;
    let t14081 = t9429 * t4549;
    let t14085 = t9429 * t4802;
    let t14086 = 0.22109259259259259258e-2 * t14085;
    let t14089 = -0.55273148148148148147e-3 * t14065 - 0.73697530864197530861e-3 * t14070 + 0.66327777777777777776e-2 * t14075 + t14051 * t300 - t14079 - 0.58958024691358024689e-2 * t10243 - 0.44218518518518518517e-2 * t14081 - 0.13345e0 * t3049 * t4978 - t14086 - 0.88437037037037037034e-2 * t10255 + 0.1621345679012345679e-1 * t10257;
    (t14075, t14078, t14081, t14085, t14089)
}
