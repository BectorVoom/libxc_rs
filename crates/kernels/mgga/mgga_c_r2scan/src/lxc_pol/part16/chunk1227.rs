//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1227/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1227<F: Float>(t29454: F, t37720: F, t11824: F, t2207: F, t3613: F, t12511: F, t6205: F, t40201: F, t40216: F, t40218: F, t41750: F, t43631: F, t43635: F, t43638: F, t43641: F, t43643: F) -> F {
    let t43645 = t37720 * t29454;
    let t43648 = t2207 * t3613 * t11824;
    let t43650 = t6205 * t12511;
    let t43652 = F::cast_from(0.10975748638225852664e0_f64) * t43631 - F::cast_from(0.95219938395347901944e-2_f64) * t40201 - F::cast_from(0.21951497276451705328e0_f64) * t43635 - F::cast_from(0.2600466522016280569e0_f64) * t43638 - F::cast_from(0.10401866088065122276e1_f64) * t43641 - F::cast_from(0.47609969197673950971e-2_f64) * t43643 - F::cast_from(0.14282990759302185292e-1_f64) * t43645 + F::cast_from(0.13099107994629972538e-1_f64) * t43648 + F::cast_from(0.86682217400542685632e-1_f64) * t43650 - t40216 - t40218 + t41750;
    t43652
}
