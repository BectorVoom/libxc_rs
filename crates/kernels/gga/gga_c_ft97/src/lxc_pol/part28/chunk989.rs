//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 989/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk989<F: Float>(t1636: F, t7386: F, t89: F, t7382: F, t1557: F, t7339: F, t1882: F, t32981: F, t32869: F, t358: F, t1570: F, t32964: F) -> (F, F, F, F, F, F, F, F, F) {
    let t139492 = t89 * t1636 * t7386;
    let t139493 = F::new(4.0) / F::new(27.0) * t139492;
    let t139495 = t89 * t1636 * t7382;
    let t139496 = F::new(8.0) / F::new(27.0) * t139495;
    let t139497 = t7339 * t1557;
    let t139507 = t1882 * t32981;
    let t139509 = t32869 * t358;
    let t139514 = t7339 * t1570;
    let t139519 = t1882 * t32964;
    (t139492, t139493, t139495, t139496, t139497, t139507, t139509, t139514, t139519)
}
