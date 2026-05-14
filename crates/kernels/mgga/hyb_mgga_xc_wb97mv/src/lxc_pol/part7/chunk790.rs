//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 790/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk790<F: Float>(t2580: F, t2585: F, t3478: F, t3520: F, t4285: F, t4297: F, t4301: F, t4305: F, t4307: F, t4312: F, t4316: F) -> (F,) {
    let t4372 = -0.1294625e1 * t4297 + 0.258925e1 * t4301 + t2580 - 0.60385e0 * t3478 + 0.905775e0 * t4285 + 0.82524375e-1 * t4305 + 0.16504875e0 * t4307 + t2585 - 0.33114e0 * t3520 + 0.248355e0 * t4312 + 0.248355e0 * t4316;
    (t4372,)
}
