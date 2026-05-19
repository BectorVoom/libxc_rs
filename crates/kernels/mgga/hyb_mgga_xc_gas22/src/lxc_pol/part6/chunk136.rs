//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 136/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk136<F: Float>(t132: F, t340: F, t394: F, t295: F, t199: F, t209: F, t303: F, t306: F, t211: F, dens_threshold: F, rho1: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t397 = piecewise3::<F>(t134, F::new(0.0), t340 * t394 / F::new(2.0));
    let t398 = t295 * sigma2;
    let t400 = F::new(1.0) + F::cast_from(0.46914023462026644e0_f64) * t199;
    let t401 = F::new(1.0) / t400;
    let t405 = t303 * t209;
    let t407 = t209 * t209;
    let t408 = t306 * t407;
    let t409 = t211 * t211;
    let t410 = F::new(1.0) / t409;
    let t412 = sigma2 * sigma2;
    (t397, t398, t400, t401, t405, t407, t408, t409, t410, t412)
}
