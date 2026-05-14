//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 766/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk766<F: Float>(t2245: F, t4192: F, t2251: F, t3317: F, t4155: F, t1352: F, t828: F) -> (F, F, F, F) {
    let t4194 = 0.16081979498692535067e2 * t2245 * t4192;
    let t4197 = t2251 - 0.34246666666666666666e-1 * t3317 + 0.5137e-1 * t4155;
    let t4202 = t1352 * t1352;
    let t4203 = t4202 * t828;
    (t4194, t4197, t4202, t4203)
}
