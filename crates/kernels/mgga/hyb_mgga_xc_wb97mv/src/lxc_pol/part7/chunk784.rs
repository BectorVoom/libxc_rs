//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 784/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk784<F: Float>(t2517: F, t4322: F, t2523: F, t3478: F, t4285: F, t1415: F, t976: F) -> (F, F, F, F) {
    let t4324 = 0.16081979498692535067e2 * t2517 * t4322;
    let t4327 = t2523 - 0.34246666666666666666e-1 * t3478 + 0.5137e-1 * t4285;
    let t4332 = t1415 * t1415;
    let t4333 = t4332 * t976;
    (t4324, t4327, t4332, t4333)
}
