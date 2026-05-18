//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1047/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1047<F: Float>(t2155: F, t26494: F, t26477: F, t209: F, t2739: F, t2740: F, t888: F, t7633: F, t7647: F, t7639: F, t695: F, t8759: F) -> (F, F, F, F, F, F, F) {
    let t26495 = t2155 * t26494;
    let t26497 = t2155 * t26477;
    let t26501 = t209 * t2739 * t888 * t2740;
    let t26502 = t2155 * t26501;
    let t26504 = t7633 * t7647;
    let t26506 = t7633 * t7639;
    let t26508 = t8759 * t695;
    (t26495, t26497, t26501, t26502, t26504, t26506, t26508)
}
