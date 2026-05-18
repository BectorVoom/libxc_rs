//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1387/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1387<F: Float>(t25520: F, t25826: F, t967: F, t21393: F, t21396: F, t21427: F, t21430: F, t21433: F, t21557: F, t21560: F, t25214: F, t25217: F, t25220: F, t29819: F) -> (F, F) {
    let t30127 = F::new(0.2069040516770936012e4) * t25826 * t25520 * t967;
    let t30137 = t21557 - F::new(0.18602370370370370371e1) * t21393 + F::new(0.39862222222222222223e0) * t21396 + t21560 + F::new(0.27385555555555555556e0) * t21430 - F::new(0.1460562962962962963e1) * t21427 + F::new(0.27385555555555555556e0) * t21433 - F::new(0.1860237037037037037e1) * t25214 + F::new(0.15944888888888888889e1) * t25217 - F::new(0.59793333333333333334e0) * t25220 + F::new(0.1898925e1) * t29819;
    (t30127, t30137)
}
