//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1110/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1110<F: Float>(t25832: F, t2787: F, t7483: F, t9225: F, t7411: F, t9229: F, t10771: F, t237: F, t732: F, t3604: F, t721: F, t1108: F, t20671: F, t9232: F, t20896: F, t9236: F) -> (F, F, F, F, F, F, F, F) {
    let t30223 = 0.48245938496077605201e2 * t25832 * t2787;
    let t30225 = 6.0 * t7483 * t9225;
    let t30227 = 0.48245938496077605201e2 * t7411 * t9229;
    let t30228 = t237 * t10771;
    let t30230 = 0.5848223622634646207e0 * t30228 * t732;
    let t30231 = t3604 * t721;
    let t30234 = 0.10526802520742363173e2 * t20671 * t1108 * t30231;
    let t30236 = 0.96491876992155210402e2 * t7411 * t9232;
    let t30238 = 0.1551780387578202009e4 * t20896 * t9236;
    (t30223, t30225, t30227, t30230, t30231, t30234, t30236, t30238)
}
