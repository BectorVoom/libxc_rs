//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 93/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk93<F: Float>(t277: F, t40: F, t243: F, t85: F, t1: F, t60: F, t132: F, t203: F, t84: F) -> (F, F, F, F) {
    let t278 = t40 * t277;
    let t279 = t243 * t85;
    let t280 = F::cast_from(0.19751673498613801407e-1_f64) * t279;
    let t281 = t60 * t1;
    let t283 = t203 * t132 * t84;
    (t278, t280, t281, t283)
}
