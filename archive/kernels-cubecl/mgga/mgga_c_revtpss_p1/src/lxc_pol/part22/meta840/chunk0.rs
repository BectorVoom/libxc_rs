//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2970/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970<F: Float>(t13981: F, t9962: F, t13951: F, t2713: F, t3964: F, t1413: F, t46835: F, t48698: F, t13845: F, t13847: F, t13848: F, t4004: F) -> (F, F, F, F) {
    let t49005 = t9962 * t13981;
    let t49008 = t3964 * t2713 * t13951;
    let t49012 = t46835 * t1413 * t48698;
    let t49016 = t13845 * t13847 * t13848 * t4004;
    (t49005, t49008, t49012, t49016)
}
