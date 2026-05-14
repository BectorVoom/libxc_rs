//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1004/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1004<F: Float>(t112: F, t843: F, t239: F, t655: F, t2339: F, t624: F, t10208: F, t68: F, t1923: F, t1927: F, t72: F, t2247: F, t38: F, t45972: F, t7342: F, t10309: F, t26178: F) -> (F, F, F, F, F, F, F, F) {
    let t94973 = t843 * t112;
    let t94975 = t239 * t655;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t95253 = 1232.0 / 81.0 * t1923 * t843 * t72 * t1927;
    let t95293 = t2247 * t38 * t239;
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    (t94973, t94975, t94978, t94982, t95253, t95293, t95316, t95319)
}
