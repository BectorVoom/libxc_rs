//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 105/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk105<F: Float>(t230: F, t251: F, t22: F, t17: F, zeta_threshold: F) -> (F, F) {
    let t253 = F::new(0.621814e-1) * t230 * t251;
    let t255 = piecewise3::<f64>(F::new(0.0) <= zeta_threshold, t22, F::new(0.0));
    let t259 = F::new(1.0) / (F::new(2.0) * t17 - F::new(2.0));
    (t253, t259)
}
