//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 812/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk812<F: Float>(t1971: F, t236: F, t36489: F, t495: F, t6108: F, t16503: F, t22971: F, t552: F, t8425: F, t14243: F, t8430: F, t1598: F, t16504: F, t8435: F, t10072: F, t34761: F) -> (F, F, F, F, F) {
    let t45175 = t36489 * t1971 * t236 * t6108 * t495;
    let t45179 = t16503 * t22971 * t552 * t8425;
    let t45183 = t16503 * t14243 * t552 * t8430;
    let t45187 = t16503 * t16504 * t1598 * t8435;
    let t45189 = t34761 * t10072;
    (t45175, t45179, t45183, t45187, t45189)
}
