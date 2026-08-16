//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2974/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974<F: Float>(t14230: F, t46802: F, t49068: F, t46888: F, t48908: F, t1398: F, t5591: F, t13946: F, t9962: F, t1413: F, t46835: F, t48694: F) -> (F, F, F, F, F) {
    let t49103 = t46802 * t49068 * t14230;
    let t49105 = t46888 * t48908;
    let t49107 = t5591 * t1398;
    let t49118 = t9962 * t13946;
    let t49121 = t46835 * t1413 * t48694;
    (t49103, t49105, t49107, t49118, t49121)
}
