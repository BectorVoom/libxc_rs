//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 748/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk748<F: Float>(t684: F, t689: F, t24278: F, t13395: F, t6014: F, t230: F, t626: F, t1418: F, t1417: F, t2409: F, t2917: F, t6045: F, t2413: F, t10915: F, t2405: F, t13522: F, t6023: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24279 = t689 * t684;
    let t24280 = t24278 * t24279;
    let t24283 = t6014 * t13395;
    let t24286 = t626 * t230;
    let t24287 = t1418 * t24286;
    let t24289 = 0.42562405586419753087e-2 * t1417 * t24287;
    let t24290 = t2917 * t2409;
    let t24291 = t6045 * t24290;
    let t24294 = t2917 * t2413;
    let t24295 = t6045 * t24294;
    let t24298 = t10915 * t2405;
    let t24299 = t6045 * t24298;
    let t24302 = t6023 * t13522;
    (t24279, t24280, t24283, t24286, t24287, t24289, t24290, t24291, t24294, t24295, t24298, t24299, t24302)
}
