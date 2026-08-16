//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 821/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk821<F: Float>(t16503: F, t35039: F, t38523: F, t7448: F, t34761: F, t9171: F, t34760: F, t8450: F, t7463: F, t3369: F, t34975: F, t38444: F, t495: F, t8440: F) -> (F, F, F, F, F) {
    let t38526 = t16503 * t35039 * t38523 * t7448;
    let t38528 = t34761 * t9171;
    let t38530 = t8450 * t34760;
    let t38531 = t38530 * t7463;
    let t38539 = t34975 * t3369 * t8440 * t38444 * t495;
    (t38526, t38528, t38530, t38531, t38539)
}
