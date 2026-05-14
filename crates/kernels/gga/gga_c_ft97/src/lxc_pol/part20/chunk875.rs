//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 875/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk875<F: Float>(t1154: F, t747: F, t6119: F, t729: F, t27819: F, t681: F, t6899: F, t89: F, t3821: F, t6008: F, t193: F, t1131: F, t24191: F, t6061: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27820 = t1154 * t747;
    let t27822 = t729 * t6119 * t27820;
    let t27823 = t27819 * t27822;
    let t27825 = t681 * t6899;
    let t27826 = t89 * t27825;
    let t27828 = t6008 * t3821;
    let t27829 = t193 * t27828;
    let t27830 = t89 * t27829;
    let t27832 = t24191 * t1131;
    let t27833 = t193 * t27832;
    let t27834 = t89 * t27833;
    let t27836 = t6061 * t1131;
    (t27820, t27822, t27823, t27826, t27828, t27830, t27832, t27834, t27836)
}
