//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2575/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2575<F: Float>(t39515: F, t4083: F, t10043: F, t9303: F, t10014: F, t10019: F, t268: F, t4101: F, t543: F, t675: F, t9890: F, t10139: F, t281: F, t4056: F, t68: F) -> (F, F, F, F, F) {
    let t47351 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t4083;
    let t47352 = t9303 * t10043;
    let t47354 = t10014 * t10019;
    let t47359 = t4101 * t268 * t675 * t9890 * t543;
    let t47364 = t10139 * t281 * t68 * t4056 * t543;
    (t47351, t47352, t47354, t47359, t47364)
}
