//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1269/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1269<F: Float>(t218: F, t675: F, t7992: F, t1167: F, t219: F, t6106: F, t2185: F, t3026: F, t7945: F, t824: F, t22260: F, t334: F) -> (F, F, F, F, F) {
    let t22265 = t218 * t675 * t7992;
    let t22269 = t218 * t219 * t6106 * t1167;
    let t22273 = t218 * t219 * t2185 * t3026;
    let t22277 = t218 * t219 * t824 * t7945;
    let t22281 = t218 * t219 * t334 * t22260;
    (t22265, t22269, t22273, t22277, t22281)
}
