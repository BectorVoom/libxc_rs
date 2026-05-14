//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 414/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk414<F: Float>(t1022: F, t1952: F, t1546: F, t89: F, t998: F, t2205: F, t2984: F, t446: F, t558: F, t925: F, t1969: F, t2993: F, t569: F, t18: F, t519: F, t3000: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3313 = t1952 * t1022;
    let t3318 = t89 * t1546 * t998;
    let t3320 = t2205 * t2984;
    let t3321 = t446 * t3320;
    let t3323 = t925 * t558;
    let t3324 = t1969 * t3323;
    let t3325 = t446 * t3324;
    let t3327 = t569 * t2993;
    let t3328 = t446 * t3327;
    let t3330 = t519 * t18;
    let t3332 = t89 * t3000 * t3330;
    (t3313, t3318, t3320, t3321, t3323, t3324, t3325, t3327, t3328, t3330, t3332)
}
