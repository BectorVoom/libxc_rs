//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1388/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1388<F: Float>(t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F, t29846: F) -> F {
    let t30150 = F::new(0.3071625e0) * t29822 - F::new(0.1898925e1) * t29825 + F::new(0.3071625e0) * t29827 + F::new(0.39862222222222222223e0) * t29757 - F::new(0.59793333333333333334e0) * t29760 + F::new(0.8969e0) * t29788 - F::new(0.3560484375e1) * t29833 + F::new(0.142419375e1) * t29836 + F::new(0.1151859375e0) * t29839 - F::new(0.76790625e-1) * t29842 + F::new(0.142419375e1) * t29844 - F::new(0.1898925e1) * t29846;
    t30150
}
