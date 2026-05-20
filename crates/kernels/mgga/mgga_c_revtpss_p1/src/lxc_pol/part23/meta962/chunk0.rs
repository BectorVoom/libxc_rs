//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3250/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3250<F: Float>(t1358: F, t212: F, t22964: F, t689: F, t13848: F, t22893: F, t47274: F, t9816: F, t22890: F, t9962: F, t13845: F, t22841: F, t73731: F, t9818: F) -> (F, F, F, F) {
    let t85509 = t689 * t212 * t22964 * t1358;
    let t85514 = t9816 * t47274 * t13848 * t22893;
    let t85516 = t9962 * t22890;
    let t85532 = t13845 * t9818 * t73731 * t22841;
    (t85509, t85514, t85516, t85532)
}
