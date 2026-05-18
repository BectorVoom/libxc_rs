//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 825/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk825<F: Float>(t3924: F, t623: F, t7275: F, t34761: F, t8447: F, t34764: F, t9147: F, t8437: F, t16503: F, t34976: F, t38422: F, t7448: F) -> (F, F, F, F, F) {
    let t38495 = t623 * t3924;
    let t38496 = t38495 * t7275;
    let t38498 = t34761 * t8447;
    let t38500 = t34764 * t9147;
    let t38502 = t34761 * t8437;
    let t38506 = t16503 * t34976 * t38422 * t7448;
    (t38496, t38498, t38500, t38502, t38506)
}
