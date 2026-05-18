//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 704/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk704<F: Float>(t598: F, t7523: F, t1083: F, t355: F, t360: F, t7458: F, t1980: F, t1988: F, t2113: F, t1131: F, t137: F, t1089: F, t1459: F) -> (F, F, F, F, F, F) {
    let t7524 = t598 * t7523;
    let t7528 = t7458 * t1083 * t355 * t360;
    let t7529 = t1980 * t7528;
    let t7531 = t1988 * t2113;
    let t7533 = t137 * t1131;
    let t7535 = t1089 * t1459 * t7533;
    (t7524, t7528, t7529, t7531, t7533, t7535)
}
