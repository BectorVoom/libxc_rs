//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1115/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1115<F: Float>(t1988: F, t9573: F, t1089: F, t13067: F, t598: F, t9552: F, t3300: F, t39066: F, t1980: F, t7458: F, t1846: F, t7712: F) -> (F, F, F, F, F) {
    let t39264 = t1988 * t9573;
    let t39268 = t598 * t1089 * t13067 * t9552;
    let t39271 = t3300 * t39066;
    let t39273 = t1980 * t7458 * t39271;
    let t39275 = t7712 * t1846;
    (t39264, t39268, t39271, t39273, t39275)
}
