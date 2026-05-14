//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 821/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk821<F: Float>(t6193: F, t852: F, t833: F, t2238: F, t831: F, t338: F) -> (F, F, F, F) {
    let t6194 = t6193 * t852;
    let t6196 = 1.0 * t833 * t6194;
    let t6198 = 1.0 / t2238 / t831;
    let t6199 = t338 * t6198;
    (t6194, t6196, t6198, t6199)
}
