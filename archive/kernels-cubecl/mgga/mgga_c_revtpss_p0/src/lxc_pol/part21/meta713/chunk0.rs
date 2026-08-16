//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2546/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2546<F: Float>(t46740: F, t9821: F, t13999: F, t9842: F, t9828: F, t9962: F, t124: F, t3923: F, t3938: F, t9816: F, t9818: F, t9769: F, t9793: F, t9794: F) -> (F, F, F, F, F, F) {
    let t46741 = t46740 * t9821;
    let t46747 = t13999 * t9842;
    let t46749 = t9962 * t9828;
    let t46751 = t124 * t3923;
    let t46754 = t9816 * t9818 * t46751 * t3938;
    let t46757 = t9793 * t9794 * t9769;
    (t46741, t46747, t46749, t46751, t46754, t46757)
}
