//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 505/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk505<F: Float>(t3218: F, t3219: F, t1021: F, t1092: F, t1093: F, t354: F) -> (F, F, F, F) {
    let t3220 = t3218 * t3219;
    let t3221 = t1021 * t3220;
    let t3222 = t1092 * t3221;
    let t3225 = F::cast_from(1.0_f64) / t1093 / t354;
    (t3220, t3221, t3222, t3225)
}
