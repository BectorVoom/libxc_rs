//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 138/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk138<F: Float>(t161: F, t413: F, t151: F, t177: F, t383: F) -> (F, F, F, F) {
    let t414 = t161 * t413;
    let t415 = t151 * t414;
    let t417 = F::cast_from(0.10003937560882938627e-2_f64) * t415 * t177;
    let t418 = t151 * t383;
    (t414, t415, t417, t418)
}
