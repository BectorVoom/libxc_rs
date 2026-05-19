//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1243/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1243<F: Float>(t10935: F, t3162: F, t3446: F, t39245: F, t39247: F, t39252: F, t39256: F, t42395: F, t42398: F, t42402: F, t42405: F, t42408: F, t42411: F, t42415: F, t42417: F, t42422: F, t42427: F) -> F {
    let t43820 = t3446 * t10935 * t3162;
    let t43823 = -t42395 + t42398 - t42402 - t42405 - t42408 - t42411 + F::cast_from(0.96056421943322389208e-3_f64) * t43820 + t39245 - F::cast_from(0.72042316457491791906e-3_f64) * t39247 + t39252 + t39256 - t42415 + t42417 + t42422 - t42427;
    t43823
}
