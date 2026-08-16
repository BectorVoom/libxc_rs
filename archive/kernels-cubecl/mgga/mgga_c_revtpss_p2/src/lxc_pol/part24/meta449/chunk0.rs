//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1412/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412<F: Float>(t5760: F, t9292: F, t40921: F, t5737: F, t4101: F, t5740: F, t9288: F, t40270: F, t1892: F, t9990: F, t1897: F, t40317: F) -> (F, F, F, F, F, F) {
    let t49172 = t9292 * t5760;
    let t49178 = t40921 * t5737;
    let t49203 = t4101 * t5740 * t9288;
    let t49210 = t40270 * t5737;
    let t49327 = t9990 * t1892;
    let t49354 = t40317 * t1897;
    (t49172, t49178, t49203, t49210, t49327, t49354)
}
