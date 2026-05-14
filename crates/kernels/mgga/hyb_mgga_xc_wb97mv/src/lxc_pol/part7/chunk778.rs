//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 778/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk778<F: Float>(t222: F, t37: F, t4283: F, t2451: F, t3478: F, t359: F, t1404: F, t3494: F, t1403: F, t957: F) -> (F, F, F, F, F, F) {
    let t4285 = t222 * t37 * t4283;
    let t4287 = t2451 - 0.35616666666666666666e-1 * t3478 + 0.53425e-1 * t4285;
    let t4289 = 0.621814e-1 * t4287 * t359;
    let t4291 = 2.0 * t3494 * t1404;
    let t4292 = t1403 * t1403;
    let t4293 = t4292 * t957;
    (t4285, t4287, t4289, t4291, t4292, t4293)
}
