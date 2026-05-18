//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 820/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk820<F: Float>(t16503: F, t16504: F, t352: F, t38416: F, t38422: F, t34761: F, t8427: F, t7467: F, t8440: F, t3369: F, t7482: F, t209: F, t34975: F, t34976: F, t495: F) -> (F, F, F, F, F) {
    let t38426 = t16503 * t16504 * t38422 * t38416 * t352;
    let t38428 = t34761 * t8427;
    let t38432 = t16503 * t16504 * t8440 * t7467;
    let t38436 = t16503 * t3369 * t8440 * t7482;
    let t38442 = t34975 * t34976 * t38422 * t209 * t352 * t495;
    (t38426, t38428, t38432, t38436, t38442)
}
