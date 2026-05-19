//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1286/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1286<F: Float>(t7: F, t27034: F, t27096: F, t27635: F, t27684: F, t27706: F, t27757: F, t27797: F, t27843: F, t10199: F, t2028: F, t10191: F, t3138: F, t8498: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t27847 = piecewise3::<F>(t9, F::new(0.0), t27034 + t27096 + t27635 + t27684 + t27706 + t27757 + t27797 + t27843);
    let t27852 = t10199 * t2028;
    let t27857 = t3138 * t8498 * t10191;
    (t27847, t27852, t27857)
}
