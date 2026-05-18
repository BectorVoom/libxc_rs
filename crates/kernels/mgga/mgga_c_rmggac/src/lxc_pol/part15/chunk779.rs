//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 779/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk779<F: Float>(t7197: F, t899: F, t271: F, t3899: F, t638: F, t641: F, t1347: F, t2128: F, t212: F, t3076: F, t672: F, t678: F) -> (F, F, F, F) {
    let t36978 = t899 * t7197;
    let t36983 = t638 * t3899 * t271 * t641;
    let t36984 = F::new(0.69557008413371175709e-2) * t36983;
    let t36992 = t1347 * t2128;
    let t37017 = t672 * t212 * t3076 * t678;
    (t36978, t36984, t36992, t37017)
}
