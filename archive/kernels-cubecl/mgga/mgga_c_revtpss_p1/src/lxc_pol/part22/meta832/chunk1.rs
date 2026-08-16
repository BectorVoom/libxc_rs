//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2955/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2955<F: Float>(t13805: F, t2661: F, t46609: F, t5608: F, t4004: F, t9934: F, t13854: F, t9962: F, t13834: F, t13999: F, t125: F, t13920: F) -> (F, F, F, F, F) {
    let t48573 = t2661 * t46609 * t5608 * t13805;
    let t48577 = t2661 * t9934 * t5608 * t4004;
    let t48591 = t9962 * t13854;
    let t48593 = t13999 * t13834;
    let t48595 = t125 * t13920;
    (t48573, t48577, t48591, t48593, t48595)
}
