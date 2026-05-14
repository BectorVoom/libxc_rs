//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1227/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1227<F: Float>(t28533: F, t2876: F, t24976: F, t6317: F, t28778: F, t99312: F, t1091: F, t2665: F, t99273: F, t1234: F, t24980: F, t2739: F, t2862: F, t6318: F, t2: F, t28719: F) -> (F, F, F, F, F, F, F) {
    let t113222 = t28533 * t2876;
    let t113224 = t6317 * t24976 * t113222;
    let t113226 = t99312 * t28778;
    let t113227 = t113226 / 3.0;
    let t113231 = t6317 * t2665 * t99273 * t1091;
    let t113236 = t24980 * t2862 * t6318 * t1234 * t2739;
    let t113238 = t2 * t28719;
    (t113222, t113224, t113226, t113227, t113231, t113236, t113238)
}
