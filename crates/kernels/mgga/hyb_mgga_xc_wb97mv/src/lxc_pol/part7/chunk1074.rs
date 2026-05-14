//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1074/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1074<F: Float>(t132: F, t10621: F, t4293: F, t7415: F, t2469: F, t4319: F, t4296: F, t7266: F, t941: F, t3502: F, t3507: F, t2480: F, t4300: F, t222: F, t4283: F, t567: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t11243 = piecewise3(t133, 0.0, t10621);
    let t11253 = 2.0 * t7415 * t4293;
    let t11255 = 1.0 * t2469 * t4319;
    let t11256 = t7266 * t4296;
    let t11257 = t11256 * t941;
    let t11259 = t3502 * t3507;
    let t11261 = t2480 * t4300;
    let t11262 = t11261 * t941;
    let t11267 = t222 * t567 * t4283;
    (t11243, t11253, t11255, t11256, t11257, t11259, t11261, t11262, t11267)
}
