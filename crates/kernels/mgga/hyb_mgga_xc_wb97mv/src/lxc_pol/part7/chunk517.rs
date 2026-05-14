//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 517/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk517<F: Float>(t2451: F, t2453: F, t2464: F, t359: F, t933: F, t937: F) -> (F, F, F) {
    let t2466 = t2451 - 0.35616666666666666666e-1 * t2453 + 0.53425e-1 * t2464;
    let t2468 = 0.621814e-1 * t2466 * t359;
    let t2469 = t933 * t937;
    (t2466, t2468, t2469)
}
