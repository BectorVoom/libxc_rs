//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 707/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk707<F: Float>(t1985: F, t838: F, t1343: F, t2048: F, t29: F, t3899: F, t3350: F, t7254: F) -> (F, F, F, F) {
    let t14249 = t1985 * t838;
    let t14267 = t2048 * t1343;
    let t14366 = t3899 * t29;
    let t16043 = t7254 * t3350;
    (t14249, t14267, t14366, t16043)
}
