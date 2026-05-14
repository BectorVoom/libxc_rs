//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 289/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk289<F: Float>(t924: F, t950: F, t931: F, t942: F, t947: F, t954: F) -> (F, F, F) {
    let t970 = 0.516475e0 * t924;
    let t973 = 0.104195e0 * t950;
    let t975 = 0.3529725e1 * t942 - t970 + 0.1549425e1 * t931 + 0.6311625e0 * t947 - t973 + 0.312585e0 * t954;
    (t970, t973, t975)
}
