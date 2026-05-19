//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 709/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk709<F: Float>(t132: F, t1019: F, t1388: F, t1445: F, t340: F, t3455: F, t3609: F, t394: F, t932: F, t1523: F, t483: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t3613 = piecewise3::<F>(t134, F::new(0.0), t1388 * t1019 / F::new(2.0) + t932 * t1445 / F::new(2.0) + t340 * t3609 / F::new(2.0) + t3455 * t394 / F::new(2.0));
    let t3616 = t1523 * t483;
    (t3613, t3616)
}
