//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 548/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk548<F: Float>(t132: F, t1019: F, t2449: F, t2624: F, t340: F, t394: F, t932: F, t221: F, t468: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t2628 = piecewise3(t134, 0.0, t2449 * t394 / 2.0 + t932 * t1019 + t340 * t2624 / 2.0);
    let t2630 = t468 * t221;
    (t2628, t2630)
}
