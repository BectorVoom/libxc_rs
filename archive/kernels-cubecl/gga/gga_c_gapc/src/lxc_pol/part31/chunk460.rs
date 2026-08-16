//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 460/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk460<F: Float>(t2493: F, t617: F, t2153: F, t285: F, t191: F, t1936: F, t320: F, t291: F, t481: F, t297: F, t875: F, t941: F) -> (F, F, F, F, F, F, F) {
    let t2494 = t617 * t2493;
    let t2497 = t2153 * t285;
    let t2498 = t2497 * t191;
    let t2501 = t320 * t1936;
    let t2502 = t481 * t291;
    let t2503 = t2502 * t297;
    let t2504 = t941 * t875;
    (t2494, t2497, t2498, t2501, t2502, t2503, t2504)
}
