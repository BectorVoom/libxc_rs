//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1350/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1350<F: Float>(t28351: F, t75638: F, t28335: F, t28392: F, t16823: F, t5737: F, t1307: F, t21827: F, t5709: F, t21868: F, t491: F, t990: F) -> (F, F, F, F, F) {
    let t103063 = t28351 * t75638;
    let t103066 = t28392 * t28335;
    let t103069 = t28351 * t16823 * t5737;
    let t103073 = t5709 * t21827 * t1307;
    let t103078 = t21868 * t491 * t990;
    (t103063, t103066, t103069, t103073, t103078)
}
