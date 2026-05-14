//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1284/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1284<F: Float>(t25520: F, t25826: F, t967: F, t21393: F, t21396: F, t21427: F, t21430: F, t21433: F, t21557: F, t21560: F, t25214: F, t25217: F, t25220: F, t29819: F, t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F, t29846: F) -> (F, F, F) {
    let t30127 = 0.2069040516770936012e4 * t25826 * t25520 * t967;
    let t30137 = t21557 - 0.18602370370370370371e1 * t21393 + 0.39862222222222222223e0 * t21396 + t21560 + 0.27385555555555555556e0 * t21430 - 0.1460562962962962963e1 * t21427 + 0.27385555555555555556e0 * t21433 - 0.1860237037037037037e1 * t25214 + 0.15944888888888888889e1 * t25217 - 0.59793333333333333334e0 * t25220 + 0.1898925e1 * t29819;
    let t30150 = 0.3071625e0 * t29822 - 0.1898925e1 * t29825 + 0.3071625e0 * t29827 + 0.39862222222222222223e0 * t29757 - 0.59793333333333333334e0 * t29760 + 0.8969e0 * t29788 - 0.3560484375e1 * t29833 + 0.142419375e1 * t29836 + 0.1151859375e0 * t29839 - 0.76790625e-1 * t29842 + 0.142419375e1 * t29844 - 0.1898925e1 * t29846;
    (t30127, t30137, t30150)
}
