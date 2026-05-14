//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1272/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1272<F: Float>(t29818: F, t957: F, t3490: F, t2484: F, t2496: F, t21462: F, t2485: F, t4247: F, t4251: F, t7009: F, t21474: F, t7025: F, t10887: F, t2490: F, t3485: F, t9135: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29822 = t957 * t29818;
    let t29824 = t3490 * t3490;
    let t29825 = t2484 * t29824;
    let t29827 = t2496 * t29824;
    let t29833 = t21462 * t4247 * t2485;
    let t29836 = t7009 * t4251 * t2485;
    let t29839 = t21474 * t4247 * t2485;
    let t29842 = t7025 * t4251 * t2485;
    let t29844 = t10887 * t2490;
    let t29846 = t3485 * t9135;
    (t29822, t29825, t29827, t29833, t29836, t29839, t29842, t29844, t29846)
}
