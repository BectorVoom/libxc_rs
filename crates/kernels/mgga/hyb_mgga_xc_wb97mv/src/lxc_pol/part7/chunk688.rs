//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 688/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk688<F: Float>(t3534: F, t956: F, t2517: F, t2453: F, t2523: F, t3478: F, t3489: F, t1408: F, t967: F) -> (F, F, F, F) {
    let t3535 = t3534 * t956;
    let t3537 = 0.16081979498692535067e2 * t2517 * t3535;
    let t3541 = t2523 - 0.17123333333333333333e-1 * t2453 - 0.17123333333333333333e-1 * t3478 + 0.5137e-1 * t3489;
    let t3544 = t1408 * t967;
    (t3535, t3537, t3541, t3544)
}
