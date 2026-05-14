//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 818/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk818<F: Float>(t732: F, t9242: F, t3625: F, t723: F, t730: F, t179: F, t780: F, t9161: F, t1123: F, t2003: F, t300: F, t2774: F, t761: F, t2031: F, t2931: F, t7700: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9244 = 0.5848223622634646207e0 * t9242 * t732;
    let t9245 = t3625 * t723;
    let t9247 = 0.35089341735807877242e1 * t730 * t9245;
    let t9253 = t179 * t780 * t9161;
    let t9257 = t2003 * t1123;
    let t9258 = t300 * t9257;
    let t9259 = t761 * t2774;
    let t9260 = t9258 * t9259;
    let t9263 = t2031 * t2931;
    let t9264 = t7700 * t9263;
    (t9244, t9245, t9247, t9253, t9257, t9258, t9259, t9260, t9263, t9264)
}
