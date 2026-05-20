//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1409/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1409<F: Float>(t1885: F, t46722: F, t1389: F, t1882: F, t46856: F, t543: F, t685: F, t72: F, t13955: F, t46946: F, t47198: F, t5665: F) -> (F, F, F, F) {
    let t48518 = t46722 * t1885;
    let t48563 = t46856 * t1389 * t1882 * t543 * t72 * t685;
    let t48600 = t46946 * t13955;
    let t48792 = t47198 * t5665;
    (t48518, t48563, t48600, t48792)
}
