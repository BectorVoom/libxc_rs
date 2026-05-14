//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 823/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk823<F: Float>(t6224: F, t881: F, t890: F, t898: F, t2316: F, t880: F) -> (F, F, F) {
    let t6226 = t881 * t6224 * t890;
    let t6228 = 0.5848223622634646207e0 * t898 * t6226;
    let t6230 = 1.0 / t2316 / t880;
    (t6226, t6228, t6230)
}
