//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 939/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk939<F: Float>(t1775: F, t20363: F, t1882: F, t20153: F, t20138: F, t20124: F, t37401: F, t89: F, t1586: F, t20098: F, t20461: F, t487: F) -> (F, F, F, F, F, F) {
    let t74287 = t1775 * t20363;
    let t74307 = t1882 * t20153;
    let t74374 = t1882 * t20138;
    let t74377 = t89 * t37401 * t20124;
    let t74389 = t1586 * t20098;
    let t74690 = t20461 * t487;
    (t74287, t74307, t74374, t74377, t74389, t74690)
}
