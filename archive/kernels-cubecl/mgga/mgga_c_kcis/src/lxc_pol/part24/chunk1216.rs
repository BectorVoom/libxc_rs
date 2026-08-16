//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1216/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1216<F: Float>(t26924: F, t29059: F, t8072: F, t95376: F, t95321: F, t19865: F, t28029: F, t13321: F, t26929: F, t5177: F, t29051: F, t92522: F) -> (F, F, F, F, F, F) {
    let t99865 = t26924 * t29059;
    let t99867 = t95376 * t8072;
    let t99869 = t95321 * t8072;
    let t99871 = t28029 * t19865;
    let t99874 = t13321 * t26929 * t5177;
    let t99876 = t92522 * t29051;
    (t99865, t99867, t99869, t99871, t99874, t99876)
}
