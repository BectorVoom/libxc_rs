//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 973/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk973<F: Float>(t10929: F, t37434: F, t37435: F, t898: F, t10648: F, t11583: F, t37453: F, t10992: F, t11563: F, t2315: F, t3446: F, t10649: F, t2482: F, t58: F, t597: F, t11584: F, t37369: F) -> (F, F, F, F, F) {
    let t39229 = t37434 * t37435 * t898 * t10929;
    let t39233 = t10648 * t37453 * t11583;
    let t39239 = t3446 * t10992 * t11563 * t2315;
    let t39244 = t10648 * t10649 * t58 * t2482 * t597;
    let t39245 = 0.72042316457491791906e-3 * t39244;
    let t39247 = t37369 * t11584;
    (t39229, t39233, t39239, t39245, t39247)
}
