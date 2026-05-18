//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 371/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk371<F: Float>(t1055: F, t1713: F, t345: F, t1734: F, t346: F, t1048: F, t1054: F, t1076: F, t1474: F, t1481: F) -> (F, F, F, F, F) {
    let t1765 = t1055 * t1713;
    let t1766 = t345 * t1765;
    let t1769 = t346 * t1734;
    let t1770 = t345 * t1769;
    let t1772 = t1048 + F::new(2.0) / F::new(3.0) * t1474 - t1054 + t1766 / F::new(2.0) - t1481 / F::new(12.0) - t1770 / F::new(4.0) + t1076;
    (t1765, t1766, t1769, t1770, t1772)
}
