//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1264/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1264<F: Float>(t22564: F, t4192: F, t22498: F, t22501: F, t22741: F, t251: F, t26298: F, t26301: F, t26304: F, t30747: F, t30750: F, t30778: F, t3333: F, t9112: F, t22404: F, t4163: F) -> (F, F, F, F) {
    let t30948 = 0.16081979498692535067e2 * t22564 * t4192;
    let t30961 = 0.621814e-1 * (t22741 - 0.11080740740740740741e0 * t22498 + 0.23744444444444444444e-1 * t22501 - 0.11080740740740740741e0 * t26298 + 0.94977777777777777776e-1 * t26301 - 0.35616666666666666666e-1 * t26304 + 0.23744444444444444444e-1 * t30747 - 0.35616666666666666666e-1 * t30750 + 0.53425e-1 * t30778) * t251;
    let t30963 = 2.0 * t3333 * t9112;
    let t30965 = 2.0 * t22404 * t4163;
    (t30948, t30961, t30963, t30965)
}
