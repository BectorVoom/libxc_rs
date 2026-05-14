//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 877/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk877<F: Float>(t7780: F, t89: F, t9055: F, t1984: F, t28: F, t558: F, t9007: F, t376: F, t9022: F, t1979: F, t7773: F, t1965: F, t37345: F, t1546: F, t9012: F, t37401: F, t9026: F) -> (F, F, F, F, F, F, F) {
    let t40301 = t89 * t7780 * t9055;
    let t40306 = t89 * t28 * t1984 * t9007 * t558;
    let t40309 = t89 * t376 * t9022;
    let t40312 = t89 * t7773 * t1979;
    let t40315 = t89 * t37345 * t1965;
    let t40318 = t89 * t1546 * t9012;
    let t40321 = t89 * t37401 * t9026;
    (t40301, t40306, t40309, t40312, t40315, t40318, t40321)
}
