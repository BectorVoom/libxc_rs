//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 922/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk922<F: Float>(t14249: F, t16503: F, t559: F, t7482: F, t16504: F, t2318: F, t34975: F, t7467: F, t1368: F, t3369: F, t7448: F, t34761: F, t9159: F) -> (F, F, F, F) {
    let t39907 = t16503 * t14249 * t559 * t7482;
    let t39911 = t34975 * t16504 * t2318 * t7467;
    let t39915 = t16503 * t3369 * t1368 * t7448;
    let t39917 = t34761 * t9159;
    (t39907, t39911, t39915, t39917)
}
