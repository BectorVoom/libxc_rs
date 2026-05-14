//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 992/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk992<F: Float>(t11584: F, t37373: F, t37426: F, t37427: F, t37428: F, t898: F, t10929: F, t37434: F, t37435: F, t10648: F, t11583: F, t37453: F, t10992: F, t11563: F, t2315: F, t3446: F) -> (F, F, F, F, F) {
    let t39221 = t37373 * t11584;
    let t39225 = t37426 * t37427 * t898 * t37428;
    let t39229 = t37434 * t37435 * t898 * t10929;
    let t39233 = t10648 * t37453 * t11583;
    let t39239 = t3446 * t10992 * t11563 * t2315;
    (t39221, t39225, t39229, t39233, t39239)
}
