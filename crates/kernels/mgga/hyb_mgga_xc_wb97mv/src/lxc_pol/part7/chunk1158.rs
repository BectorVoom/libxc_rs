//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1158/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1158<F: Float>(t1099: F, t2683: F, t2690: F, t7531: F, t7535: F, t2772: F, t7591: F, t2712: F, t2785: F, t2808: F, t1056: F, t458: F, t7696: F, t2709: F, t1046: F, t7779: F) -> (F, F, F, F, F, F, F, F) {
    let t24008 = 0.61524113149298439947e4 * t1099 * t7531 * t2690 * t7535 * t2683;
    let t24015 = 0.62337092780453269531e3 * t1099 * t7591 * t2683 * t2772;
    let t24016 = t2712 * t2785;
    let t24019 = t2712 * t2808;
    let t24022 = t458 * t1056 * t7696;
    let t24024 = t2709 * t2785;
    let t24026 = t2709 * t2808;
    let t24028 = t1046 * t7779;
    (t24008, t24015, t24016, t24019, t24022, t24024, t24026, t24028)
}
