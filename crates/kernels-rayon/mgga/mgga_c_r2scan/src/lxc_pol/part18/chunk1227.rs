//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1227/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1227(t29454: f64, t37720: f64, t11824: f64, t2207: f64, t3613: f64, t12511: f64, t6205: f64, t40201: f64, t40216: f64, t40218: f64, t41750: f64, t43631: f64, t43635: f64, t43638: f64, t43641: f64, t43643: f64) -> f64 {
    let t43645 = t37720 * t29454;
    let t43648 = t2207 * t3613 * t11824;
    let t43650 = t6205 * t12511;
    let t43652 = 0.10975748638225852664e0_f64 * t43631 - 0.95219938395347901944e-2_f64 * t40201 - 0.21951497276451705328e0_f64 * t43635 - 0.2600466522016280569e0_f64 * t43638 - 0.10401866088065122276e1_f64 * t43641 - 0.47609969197673950971e-2_f64 * t43643 - 0.14282990759302185292e-1_f64 * t43645 + 0.13099107994629972538e-1_f64 * t43648 + 0.86682217400542685632e-1_f64 * t43650 - t40216 - t40218 + t41750;
    t43652
}
