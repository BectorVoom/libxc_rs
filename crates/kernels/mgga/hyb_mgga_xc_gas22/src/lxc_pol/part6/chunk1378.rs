//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1378/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1378<F: Float>(t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F, t29846: F) -> F {
    let t29945 = F::new(0.6311625e0) * t29822 - F::new(0.3529725e1) * t29825 + F::new(0.6311625e0) * t29827 + F::cast_from(0.68863333333333333333e0_f64) * t29757 - F::new(0.103295e1) * t29760 + F::new(0.1549425e1) * t29788 - F::cast_from(0.6618234375e1_f64) * t29833 + F::cast_from(0.264729375e1_f64) * t29836 + F::cast_from(0.2366859375e0_f64) * t29839 - F::cast_from(0.157790625e0_f64) * t29842 + F::cast_from(0.264729375e1_f64) * t29844 - F::new(0.3529725e1) * t29846;
    t29945
}
