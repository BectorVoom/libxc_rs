//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1081/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1081<F: Float>(t40201: F, t40216: F, t40218: F, t41750: F, t43631: F, t43635: F, t43638: F, t43641: F, t43643: F, t43645: F, t43648: F, t43650: F, t2124: F, t29496: F, t39849: F, t11670: F, t29500: F) -> (F, F, F) {
    let t43652 = 0.10975748638225852664e0 * t43631 - 0.95219938395347901944e-2 * t40201 - 0.21951497276451705328e0 * t43635 - 0.2600466522016280569e0 * t43638 - 0.10401866088065122276e1 * t43641 - 0.47609969197673950971e-2 * t43643 - 0.14282990759302185292e-1 * t43645 + 0.13099107994629972538e-1 * t43648 + 0.86682217400542685632e-1 * t43650 - t40216 - t40218 + t41750;
    let t43654 = t39849 * t2124 * t29496;
    let t43657 = t11670 * t2124 * t29500;
    (t43652, t43654, t43657)
}
