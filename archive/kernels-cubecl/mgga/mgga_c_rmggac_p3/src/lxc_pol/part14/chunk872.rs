//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 872/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk872<F: Float>(t39171: F, t7720: F, t236: F, t495: F, t7230: F, t7248: F, t9182: F, t2144: F, t3351: F, t3352: F, t5263: F, t1596: F, t1986: F) -> (F, F, F, F) {
    let t39172 = t7720 * t39171;
    let t39177 = t7230 * t7248 * t236 * t9182 * t495;
    let t39181 = t3351 * t3352 * t2144 * t5263;
    let t39183 = t1986 * t1596;
    (t39172, t39177, t39181, t39183)
}
