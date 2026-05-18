//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 573/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk573<F: Float>(t1105: F, t2707: F, t1029: F, t450: F, t441: F, t1046: F) -> (F, F, F, F, F) {
    let t2709 = F::new(0.24415263074675393405e-3) * t1105 * t2707;
    let t2710 = t1029 * t450;
    let t2711 = F::new(1.0) / t2710;
    let t2712 = t441 * t2711;
    let t2713 = t1046 * t1046;
    (t2709, t2710, t2711, t2712, t2713)
}
