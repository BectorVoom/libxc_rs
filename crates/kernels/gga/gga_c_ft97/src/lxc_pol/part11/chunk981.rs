//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 981/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk981<F: Float>(t43350: F, t43351: F, t446: F, t2409: F, t2739: F, t2665: F, t824: F, t9578: F, t10409: F, t10411: F, t1882: F, t10406: F, t2413: F, t2682: F, t10248: F, t9587: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43353 = t446 * t43350 * t43351;
    let t43355 = t2409 * t2739;
    let t43357 = t446 * t2665 * t43355;
    let t43359 = t9578 * t824;
    let t43361 = t446 * t10409 * t43359;
    let t43363 = t1882 * t10411;
    let t43365 = t1882 * t10406;
    let t43367 = t2413 * t2682;
    let t43369 = t446 * t10248 * t43367;
    let t43371 = t9587 * t824;
    (t43353, t43355, t43357, t43359, t43361, t43363, t43365, t43367, t43369, t43371)
}
