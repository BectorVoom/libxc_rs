//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 796/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk796<F: Float>(t408: F, t4415: F, t416: F, t421: F, t420: F, t1447: F, t424: F, t1479: F, t1291: F, t1466: F, t1471: F, t1477: F, t1485: F, t4088: F, t4106: F, t412: F, t415: F, t423: F, t428: F, t4406: F, t4409: F, t4412: F, tau1: F) -> (F,) {
    let t4416 = t408 * tau1;
    let t4417 = t4415 * t4416;
    let t4421 = 1.0 / t421 / t416;
    let t4422 = t420 * t4421;
    let t4431 = t424 * t1447;
    let t4432 = t4431 * t1479;
    let t4444 = 0.26666666666666666666e0 * t4406 * t4409 + 0.7111111111111111111e0 * t4412 * t4409 + 0.17066666666666666667e0 * t1477 * t4417 + 0.44444444444444444444e0 * t4422 * t4409 + 0.17066666666666666667e0 * t1485 * t4417 + 0.576e0 * t423 * t424 * t4088 * t428 - 0.99555555555555555556e0 * t1477 * t4432 - 0.99555555555555555556e0 * t1485 * t4432 - 40.0 / 9.0 * t412 * t4106 - 80.0 / 9.0 * t1466 * t4106 - 80.0 / 9.0 * t415 * t1471 * t1291;
    (t4444,)
}
