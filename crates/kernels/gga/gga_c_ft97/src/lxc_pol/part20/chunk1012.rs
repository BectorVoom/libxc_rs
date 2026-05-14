//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1012/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1012<F: Float>(t24670: F, t8392: F, t24739: F, t6094: F, t8232: F, t1882: F, t24830: F, t24801: F, t24834: F, t24808: F, t24702: F, t24587: F, t2567: F, t6148: F, t24650: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t97261 = t8392 * t24670;
    let t97267 = t8392 * t24739;
    let t97269 = t8232 * t6094;
    let t97271 = t1882 * t24830;
    let t97273 = t1882 * t24801;
    let t97275 = t1882 * t24834;
    let t97277 = t1882 * t24808;
    let t97283 = t1882 * t24702;
    let t97285 = t1882 * t24587;
    let t97299 = t6148 * t2567;
    let t97304 = t24650 * t761;
    (t97261, t97267, t97269, t97271, t97273, t97275, t97277, t97283, t97285, t97299, t97304)
}
