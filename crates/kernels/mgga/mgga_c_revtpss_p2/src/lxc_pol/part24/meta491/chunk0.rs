//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1487/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1487<F: Float>(t22021: F, t9793: F, t9794: F, t6876: F, t9909: F, t22026: F, t46929: F, t808: F, t22259: F, t9976: F, t22125: F, t2713: F, t3964: F) -> (F, F, F, F, F) {
    let t74341 = t9793 * t9794 * t22021;
    let t74358 = t9909 * t6876;
    let t74362 = t46929 * t808 * t22026;
    let t74429 = t9976 * t22259;
    let t74437 = t3964 * t2713 * t22125;
    (t74341, t74358, t74362, t74429, t74437)
}
