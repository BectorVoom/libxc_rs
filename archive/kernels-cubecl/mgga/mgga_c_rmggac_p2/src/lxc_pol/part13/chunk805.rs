//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 805/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk805<F: Float>(t2231: F, t934: F, t36504: F, t36527: F, t1347: F, t2232: F, t4793: F, t703: F, t275: F, t8198: F, t36700: F, t36752: F) -> (F, F, F, F, F, F, F, F) {
    let t37950 = t934 * t2231;
    let t37964 = F::cast_from(0.13659505348792789029e1_f64) * t36504;
    let t37976 = F::cast_from(0.2439011983326002265e-2_f64) * t36527;
    let t38029 = t1347 * t2232;
    let t38031 = t4793 * t703;
    let t38036 = t275 * t8198;
    let t38047 = F::cast_from(0.18292589874945016987e-2_f64) * t36700;
    let t38060 = F::cast_from(0.30487649791575028312e-3_f64) * t36752;
    (t37950, t37964, t37976, t38029, t38031, t38036, t38047, t38060)
}
