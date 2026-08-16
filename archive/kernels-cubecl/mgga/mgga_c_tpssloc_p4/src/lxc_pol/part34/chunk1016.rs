//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1016/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1016<F: Float>(t2085: F, t3787: F, t22923: F, t22925: F, t193: F, t201: F, t2056: F) -> (F, F, F, F) {
    let t24127 = t3787 * t2085;
    let t24156 = F::cast_from(0.12793931631041761173e0_f64) * t22923;
    let t24157 = F::cast_from(0.52089578783527170489e-1_f64) * t22925;
    let t24191 = t193 * t201 * t2056;
    (t24127, t24156, t24157, t24191)
}
