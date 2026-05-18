//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 213/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk213<F: Float>(t276: F, t282: F, t1: F, t791: F, t315: F, t468: F, t122: F) -> (F, F, F, F, F) {
    let t792 = t276 * t282;
    let t793 = t792 * t1;
    let t794 = t791 * t793;
    let t795 = t468 * t315;
    let t798 = t792 * t122;
    (t792, t793, t794, t795, t798)
}
