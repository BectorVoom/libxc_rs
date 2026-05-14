//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 699/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk699<F: Float>(t1298: F, t192: F, t1674: F, t301: F, t922: F, t495: F, t96: F, t1268: F, t1679: F, t1680: F, t2831: F, t694: F, t1670: F, t839: F, t1427: F, t695: F) -> (F, F, F, F, F, F, F) {
    let t5407 = t192 * t1298;
    let t5409 = t1674 * t5407 * t301;
    let t5412 = t922 * t192;
    let t5414 = t96 * t5412 * t495;
    let t5417 = t1679 * t1680 * t1268;
    let t5419 = t694 * t2831 * t495;
    let t5422 = t694 * t1670 * t839;
    let t5425 = t1674 * t695 * t1427;
    (t5409, t5412, t5414, t5417, t5419, t5422, t5425)
}
