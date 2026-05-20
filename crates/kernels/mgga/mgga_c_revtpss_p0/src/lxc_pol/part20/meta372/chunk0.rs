//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1352/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352<F: Float>(t10867: F, t860: F, t2722: F, t2723: F, t10069: F, t10929: F, t138: F, t785: F, t9302: F, t2786: F, t10073: F, t10920: F) -> (F, F, F, F, F, F, F) {
    let t40258 = t10867 * t860;
    let t40262 = t2722 * t2722;
    let t40263 = t40262 * t2723;
    let t40267 = t10069 * t10929;
    let t40270 = t138 * t9302 * t785;
    let t40271 = t40270 * t2786;
    let t40273 = t10073 * t10920;
    (t40258, t40262, t40263, t40267, t40270, t40271, t40273)
}
