//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 782/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk782<F: Float>(t132: F, t1388: F, t1445: F, t340: F, t394: F, t4224: F, t4348: F, t1493: F, t416: F, t418: F, t196: F, t413: F, t3955: F, t195: F, t421: F, t423: F, t1478: F, t295: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t4352 = piecewise3(t134, 0.0, t4224 * t394 / 2.0 + t1388 * t1445 + t340 * t4348 / 2.0);
    let t4356 = 1.0 / t1493;
    let t4361 = t418 * t416;
    let t4363 = 1.0 / t196 / t4361;
    let t4368 = t413 * t413;
    let t4369 = t418 * t3955;
    let t4371 = 1.0 / t195 / t4369;
    let t4373 = t421 * t421;
    let t4374 = 1.0 / t4373;
    let t4383 = t4356 * t423;
    let t4389 = t295 * t1478;
    (t4352, t4356, t4363, t4368, t4371, t4374, t4383, t4389)
}
