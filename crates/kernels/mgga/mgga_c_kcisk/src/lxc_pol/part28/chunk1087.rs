//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1087/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1087<F: Float>(t5507: F, t9226: F, t2023: F, t7261: F, t7591: F, t7602: F, t7581: F, t4998: F, t9217: F, t2013: F, t9168: F, t12180: F, t18260: F, t18272: F, t2638: F, t5471: F, t7619: F, t9218: F) -> (F,) {
    let t24903 = t5507 * t9226;
    let t24904 = t24903 * t2023;
    let t24905 = t7261 * t24904;
    let t24908 = t7591 * t7602;
    let t24910 = t7581 * t7602;
    let t24912 = t4998 * t9217;
    let t24913 = t2013 * t24912;
    let t24920 = t4998 * t9168;
    let t24921 = t2013 * t24920;
    let t24923 = -0.35981577432354634426e-1 * t7581 * t7619 - 0.47975436576472845902e-1 * t18260 * t2638 + 0.95950873152945691804e-1 * t7591 * t7619 + 0.5397236614853195164e-1 * t2013 * t24905 - 0.15991812192157615301e-1 * t24908 + 0.59969295720591057377e-2 * t24910 - 0.59969295720591057377e-2 * t24913 - 0.17990788716177317213e-1 * t5471 * t9218 + 0.17990788716177317213e-1 * t18272 * t2638 - 0.19989765240197019126e-2 * t12180 + 0.29984647860295528689e-2 * t24921;
    (t24923,)
}
