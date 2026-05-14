//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 684/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk684<F: Float>(t745: F, t9385: F, t9368: F, t2514: F, t746: F, t2495: F, t744: F, t2576: F, t2582: F, t2584: F, t700: F) -> (F, F, F, F, F) {
    let t9485 = t9385 * t745;
    let t9488 = t9368 * t745;
    let t9501 = t746 * t2514;
    let t9507 = t2514 * t2495;
    let t9508 = t9507 * t744;
    let t9514 = 0.48245938496077605201e2 * t2582 * t2576 * t2584 * t700;
    (t9485, t9488, t9501, t9508, t9514)
}
