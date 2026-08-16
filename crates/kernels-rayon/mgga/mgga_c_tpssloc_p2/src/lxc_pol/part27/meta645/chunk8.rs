//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2215/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215(t23418: f64, t4669: f64, t13765: f64, t23419: f64, t14033: f64, t14069: f64, t14488: f64, t23457: f64, t23495: f64, t25585: f64, t25589: f64, t25609: f64, t3073: f64, t360: f64, t4575: f64, t6723: f64, t6735: f64, t6742: f64, t6744: f64, t68: f64, t7574: f64, t7578: f64, t83041: f64, t83046: f64, t83220: f64) -> f64 {
    let t88513 = t4669 * t23418;
    let t88517 = t23419 * t13765 / 1728.0_f64;
    let t88533 = -0.20186378047070195428e-3_f64 * t25589 * t6735 + 0.16149102437656156342e-2_f64 * t23457 * t7578 + 0.16149102437656156342e-2_f64 * t6723 * t25609 + 0.16149102437656156342e-2_f64 * t25585 * t6735 + t88513 * t3073 / 1152.0_f64 + t88517 - t83220 * t4575 / 216.0_f64 + 0.10093189023535097714e-3_f64 * t6742 * t6744 * t14488 * t68 * t360 + t23419 * t14069 / 1152.0_f64 + t23419 * t14033 / 2304.0_f64 + t83041 / 1728.0_f64 - t83046 / 216.0_f64 - 0.10093189023535097714e-3_f64 * t7574 * t23495;
    t88533
}
