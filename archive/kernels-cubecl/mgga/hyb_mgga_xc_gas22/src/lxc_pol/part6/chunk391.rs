//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 391/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk391<F: Float>(t132: F, t1388: F, t1445: F, t340: F, t394: F, t295: F, t412: F, t420: F, t303: F, t209: F, t306: F, t211: F, t409: F, dens_threshold: F, rho1: F, tau1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t1449 = piecewise3::<F>(t134, F::cast_from(0.0_f64), t1388 * t394 / F::cast_from(2.0_f64) + t340 * t1445 / F::cast_from(2.0_f64));
    let t1454 = t295 * t412;
    let t1455 = F::cast_from(1.0_f64) / t420;
    let t1459 = t303 * tau1;
    let t1464 = t306 * t209;
    let t1467 = t409 * t211;
    (t1449, t1454, t1455, t1459, t1464, t1467)
}
