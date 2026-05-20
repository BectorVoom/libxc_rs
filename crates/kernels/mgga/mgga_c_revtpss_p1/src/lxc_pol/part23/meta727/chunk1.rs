//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2495/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2495<F: Float>(t213: F, t225: F, t46475: F, t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t10115: F, t1894: F, t14094: F, t2435: F) -> (F, F, F, F, F) {
    let t49439 = t213 * t225 * t46475;
    let t49468 = t9292 * t5600;
    let t49471 = t786 * t1893 * t4075;
    let t49474 = t10115 * t1894;
    let t49476 = t2435 * t14094;
    (t49439, t49468, t49471, t49474, t49476)
}
