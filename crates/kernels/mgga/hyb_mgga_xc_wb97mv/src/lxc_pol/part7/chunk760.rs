//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 760/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk760<F: Float>(t222: F, t37: F, t4153: F, t2176: F, t3317: F, t251: F, t1341: F, t3333: F, t1340: F, t809: F) -> (F, F, F, F, F, F) {
    let t4155 = t222 * t37 * t4153;
    let t4157 = t2176 - 0.35616666666666666666e-1 * t3317 + 0.53425e-1 * t4155;
    let t4159 = 0.621814e-1 * t4157 * t251;
    let t4161 = 2.0 * t3333 * t1341;
    let t4162 = t1340 * t1340;
    let t4163 = t4162 * t809;
    (t4155, t4157, t4159, t4161, t4162, t4163)
}
