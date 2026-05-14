//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 798/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk798<F: Float>(t4097: F, t422: F, t1451: F, t1452: F, t1457: F, t1461: F, t1466: F, t1616: F, t397: F, t398: F, t408: F, t4094: F, t4099: F, t4103: F, t412: F, t415: F, t423: F, t4447: F, t4454: F, t4459: F, t4463: F, t4469: F, t4472: F, t4475: F, t4478: F) -> (F,) {
    let t4487 = t422 * t4097;
    let t4491 = -0.25173333333333333333e0 * t423 * t397 * t4447 * t408 + 0.27306666666666666666e-1 * t423 * t1451 * t4454 * t1457 - 0.85333333333333333333e-1 * t1452 * t4459 + 0.91022222222222222219e-2 * t4463 * t4469 + 50.0 / 9.0 * t4472 * t4099 + 50.0 / 9.0 * t4475 * t4099 + 0.19911111111111111112e0 * t398 * t4478 - 40.0 / 9.0 * t1461 * t4094 + 50.0 / 9.0 * t412 * t4103 + 200.0 / 9.0 * t1466 * t4103 + 50.0 / 3.0 * t415 * t4487 * t1616;
    (t4491,)
}
