//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1759/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759<F: Float>(t4059: F, t9909: F, t9812: F, t9962: F, t13845: F, t46751: F, t9818: F, t9835: F, t13847: F, t9819: F, t9840: F, t9958: F) -> (F, F, F, F, F) {
    let t47229 = t9909 * t4059;
    let t47231 = t9962 * t9812;
    let t47235 = t13845 * t9818 * t46751 * t9835;
    let t47239 = t13845 * t13847 * t9819 * t9840;
    let t47245 = t9962 * t9958;
    (t47229, t47231, t47235, t47239, t47245)
}
