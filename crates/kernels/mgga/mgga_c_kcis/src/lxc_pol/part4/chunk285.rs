//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 285/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk285<F: Float>(t1056: F, t922: F, t251: F, t736: F, t323: F, t325: F, t253: F) -> (F, F, F, F) {
    let t1057 = t1056 * t922;
    let t1060 = t736 * t251;
    let t1063 = 0.7925e-3 * t323 * t1060 * t325;
    let t1064 = t251 * t253;
    (t1057, t1060, t1063, t1064)
}
