//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1114/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1114<F: Float>(t1089: F, t4643: F, t598: F, t8484: F, t1980: F, t38798: F, t7458: F, t5676: F, t570: F, t6171: F, t1750: F, t31824: F) -> (F, F, F, F, F) {
    let t39240 = t598 * t1089 * t4643 * t8484;
    let t39243 = t1980 * t7458 * t38798;
    let t39254 = t570 * t5676;
    let t39256 = t570 * t6171;
    let t39262 = t31824 * t1750;
    (t39240, t39243, t39254, t39256, t39262)
}
