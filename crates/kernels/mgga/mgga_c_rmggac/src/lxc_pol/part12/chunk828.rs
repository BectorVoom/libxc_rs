//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 828/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk828<F: Float>(t3369: F, t34975: F, t38444: F, t495: F, t8440: F, t34761: F, t8422: F, t16503: F, t35039: F, t7461: F, t16504: F, t38416: F) -> (F, F, F, F) {
    let t38539 = t34975 * t3369 * t8440 * t38444 * t495;
    let t38541 = t34761 * t8422;
    let t38545 = t16503 * t35039 * t8440 * t7461;
    let t38550 = t34975 * t16504 * t8440 * t38416 * t495;
    (t38539, t38541, t38545, t38550)
}
