//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1140/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1140<F: Float>(t7375: F, t933: F, t23177: F, t7315: F, t982: F, t23116: F, t376: F, t23078: F, t2526: F, t2555: F, t2565: F, t2594: F, t7359: F, t963: F, t2532: F, t2554: F, t363: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23305 = t933 * t7375;
    let t23345 = 0.18467901234567901234e0 * t23177;
    let t23363 = t982 * t7315;
    let t23366 = t376 * t23116;
    let t23373 = t376 * t23078;
    let t23379 = t2526 * t2555;
    let t23392 = t2565 * t2594;
    let t23395 = t963 * t7359;
    let t23400 = t363 / t2554 / t2532;
    (t23305, t23345, t23363, t23366, t23373, t23379, t23392, t23395, t23400)
}
