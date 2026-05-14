//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 355/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk355<F: Float>(t143: F, t1246: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F, t1245: F) -> (F, F, F, F, F, F, F, F) {
    let t145 = 0.135e1 < t143;
    let t1249 = t711 * t1246;
    let t1251 = t715 * t1246;
    let t1253 = t719 * t1246;
    let t1255 = t723 * t1246;
    let t1257 = t727 * t1246;
    let t1259 = t731 * t1246;
    let t1261 = t735 * t1246;
    let t1264 = piecewise3(t145, 0.0, t1245);
    (t1249, t1251, t1253, t1255, t1257, t1259, t1261, t1264)
}
