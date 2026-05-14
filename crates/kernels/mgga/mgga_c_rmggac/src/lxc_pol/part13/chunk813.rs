//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 813/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk813<F: Float>(t270: F, t38843: F, t7349: F, t7351: F, t2019: F, t2339: F, t7926: F, t2010: F, t2415: F, t4018: F, t8342: F, t938: F, t333: F, t511: F, t7230: F, t7231: F, t8666: F) -> (F, F, F, F, F) {
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    let t40214 = t2010 * t2415 * t4018;
    let t40217 = t2010 * t8342 * t938;
    let t40222 = t7230 * t7231 * t511 * t8666 * t333;
    (t40198, t40201, t40214, t40217, t40222)
}
