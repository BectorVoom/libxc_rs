//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 661/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk661<F: Float>(t2240: F, t3769: F, t2246: F, t3017: F, t3732: F, t1196: F, t871: F) -> (F, F, F, F) {
    let t3771 = F::cast_from(0.16081979498692535067e2_f64) * t2240 * t3769;
    let t3774 = t2246 - F::cast_from(0.34246666666666666666e-1_f64) * t3017 + F::new(0.5137e-1) * t3732;
    let t3779 = t1196 * t1196;
    let t3780 = t3779 * t871;
    (t3771, t3774, t3779, t3780)
}
