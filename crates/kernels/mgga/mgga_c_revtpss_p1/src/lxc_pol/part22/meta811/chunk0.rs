//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2914/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914<F: Float>(t39515: F, t4083: F, t10043: F, t9303: F, t10139: F, t281: F, t4056: F, t543: F, t68: F, t14192: F, t555: F, t10115: F, t1441: F) -> (F, F, F, F, F) {
    let t47351 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t4083;
    let t47352 = t9303 * t10043;
    let t47364 = t10139 * t281 * t68 * t4056 * t543;
    let t47371 = t14192 * t555;
    let t47381 = t10115 * t1441;
    (t47351, t47352, t47364, t47371, t47381)
}
