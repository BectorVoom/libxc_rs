//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 59/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk59<F: Float>(t125: F, t138: F, t86: F, t60: F, t82: F) -> (F, F, F) {
    let t140 = t86 * t125 * t138;
    let t142 = t60 * t82 + F::cast_from(0.99491666666666666664e-2_f64) * t140;
    let t143 = F::sqrt(F::cast_from(4.0_f64));
    (t140, t142, t143)
}
