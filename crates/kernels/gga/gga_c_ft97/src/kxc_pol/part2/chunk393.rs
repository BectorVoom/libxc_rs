//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 393/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk393<F: Float>(t2157: F, t605: F, t144: F, t161: F, t1637: F, t89: F, t1882: F, t576: F, t611: F, t558: F, t574: F, t616: F) -> (F, F, F, F, F, F) {
    let t2158 = t605 * t2157;
    let t2159 = t144 * t2158;
    let t2164 = F::new(4.0) / F::new(27.0) * t89 * t1637 * t161;
    let t2165 = t1882 * t576;
    let t2167 = t1882 * t611;
    let t2170 = t574 * t616 * t558;
    (t2158, t2159, t2164, t2165, t2167, t2170)
}
