//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 916/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk916<F: Float>(t36471: F, t5166: F, t656: F, t5011: F, t511: F, t2136: F, t270: F, t38843: F, t7349: F, t7351: F, t2019: F, t2339: F, t7926: F) -> (F, F, F, F) {
    let t40191 = t36471 * t656 * t5166;
    let t40193 = t5011 * t511;
    let t40194 = t40193 * t2136;
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    (t40191, t40194, t40198, t40201)
}
