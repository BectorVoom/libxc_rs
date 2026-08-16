//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 721/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk721<F: Float>(t117: F, t5011: F, t10112: F, t6349: F, t2000: F, t326: F, t1985: F, t797: F, t838: F, t1343: F, t2048: F, t29: F, t3899: F) -> (F, F, F, F, F, F, F, F) {
    let t11905 = t5011 * t117;
    let t12970 = t10112 * t117;
    let t13283 = t6349 * t117;
    let t14237 = t2000 * t326;
    let t14243 = t1985 * t797;
    let t14249 = t1985 * t838;
    let t14267 = t2048 * t1343;
    let t14366 = t3899 * t29;
    (t11905, t12970, t13283, t14237, t14243, t14249, t14267, t14366)
}
