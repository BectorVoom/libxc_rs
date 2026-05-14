//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 843/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk843<F: Float>(t1691: F, t408: F, t32129: F, t5608: F, t136815: F, t373: F, t32238: F, t7837: F, t32339: F, t376: F, t89: F, t32343: F, t1557: F, t7211: F, t174: F, t2248: F) -> (F, F, F, F, F, F, F, F) {
    let t137028 = t408 * t1691;
    let t137035 = t32129 * t5608;
    let t137037 = t136815 * t373;
    let t137047 = t7837 * t32238;
    let t137070 = t89 * t376 * t32339;
    let t137073 = t89 * t376 * t32343;
    let t137082 = t7211 * t1557;
    let t137087 = t2248 * t174;
    (t137028, t137035, t137037, t137047, t137070, t137073, t137082, t137087)
}
