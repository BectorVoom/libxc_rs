//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1489/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1489<F: Float>(t6846: F, t9909: F, t1413: F, t46835: F, t74483: F, t22061: F, t9793: F, t9794: F, t22026: F, t46802: F, t46694: F, t6850: F) -> (F, F, F, F, F) {
    let t74585 = t9909 * t6846;
    let t74638 = t46835 * t1413 * t74483;
    let t74641 = t9793 * t9794 * t22061;
    let t74677 = t46802 * t9794 * t22026;
    let t74682 = t46694 * t6850;
    (t74585, t74638, t74641, t74677, t74682)
}
