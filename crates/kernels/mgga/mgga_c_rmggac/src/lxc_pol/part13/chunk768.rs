//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 768/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk768<F: Float>(t2144: F, t3351: F, t3352: F, t5263: F, t1596: F, t1986: F, t7720: F, t495: F, t515: F, t7230: F, t8377: F, t511: F, t5169: F, t9188: F, t5260: F, t1594: F) -> (F, F, F, F, F, F) {
    let t39181 = t3351 * t3352 * t2144 * t5263;
    let t39183 = t1986 * t1596;
    let t39184 = t7720 * t39183;
    let t39189 = t7230 * t3352 * t515 * t8377 * t495;
    let t39193 = t3351 * t9188 * t511 * t5169;
    let t39197 = t3351 * t9188 * t515 * t5260;
    let t39199 = t1986 * t1594;
    (t39181, t39184, t39189, t39193, t39197, t39199)
}
