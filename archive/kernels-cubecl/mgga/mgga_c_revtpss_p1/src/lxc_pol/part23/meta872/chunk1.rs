//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2773/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2773<F: Float>(t221: F, t22279: F, t3978: F, t9921: F, t22255: F, t3930: F, t22259: F, t9976: F, t22125: F, t2713: F, t3964: F, t13848: F, t22096: F, t9816: F, t9818: F) -> (F, F, F, F, F) {
    let t74423 = t221 * t22279;
    let t74425 = t3978 * t9921 * t74423;
    let t74427 = t3930 * t22255;
    let t74429 = t9976 * t22259;
    let t74437 = t3964 * t2713 * t22125;
    let t74461 = t9816 * t9818 * t13848 * t22096;
    (t74425, t74427, t74429, t74437, t74461)
}
