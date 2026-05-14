//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1254/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1254<F: Float>(t24437: F, t24438: F, t31024: F, t684: F, t122215: F, t27762: F, t6118: F, t1131: F, t27742: F, t1434: F, t193: F, t2506: F, t24448: F, t5092: F, t6061: F, t743: F) -> (F, F, F, F, F) {
    let t124093 = t24437 * t24438 * t31024 * t684;
    let t124096 = t6118 * t27762 * t122215;
    let t124098 = t27742 * t1131;
    let t124101 = t1434 * t193 * t2506 * t124098;
    let t124106 = t24448 * t193 * t743 * t6061 * t5092;
    (t124093, t124096, t124098, t124101, t124106)
}
