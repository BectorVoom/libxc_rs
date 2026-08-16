//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 130/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk130<F: Float>(t155: F, t422: F, t181: F, t388: F, t156: F, t2: F, t180: F, t243: F, t245: F) -> (F, F, F, F) {
    let t423 = t155 * t422;
    let t425 = F::cast_from(0.19751673498613801407e-1_f64) * t388 * t181;
    let t426 = t156 * t2;
    let t428 = t243 * t245 * t180;
    (t423, t425, t426, t428)
}
