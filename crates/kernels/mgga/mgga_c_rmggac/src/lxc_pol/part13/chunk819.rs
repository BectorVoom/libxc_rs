//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 819/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk819<F: Float>(t34764: F, t9147: F, t34761: F, t8437: F, t16503: F, t34976: F, t38422: F, t7448: F, t118: F, t7417: F, t2281: F, t7461: F) -> (F, F, F, F) {
    let t38500 = t34764 * t9147;
    let t38502 = t34761 * t8437;
    let t38506 = t16503 * t34976 * t38422 * t7448;
    let t38508 = t7417 * t118;
    let t38511 = t16503 * t38508 * t2281 * t7461;
    (t38500, t38502, t38506, t38511)
}
