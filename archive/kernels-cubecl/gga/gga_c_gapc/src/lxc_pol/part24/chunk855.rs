//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 855/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk855<F: Float>(t10153: F, t10252: F, t268: F, t8508: F, t6853: F, t2210: F, t6857: F, t10243: F, t276: F, t6194: F, t10246: F, t2902: F) -> (F, F, F, F, F) {
    let t10253 = t10153 * t10252;
    let t10255 = t8508 * t268;
    let t10256 = t10255 * t6853;
    let t10257 = t2210 * t6857;
    let t10258 = t10256 * t10257;
    let t10260 = t10243 * t276;
    let t10261 = t10260 * t6194;
    let t10262 = t10261 * t10246;
    let t10264 = t2902 * t268;
    (t10253, t10256, t10258, t10262, t10264)
}
