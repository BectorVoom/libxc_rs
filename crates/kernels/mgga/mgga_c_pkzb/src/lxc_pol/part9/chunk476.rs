//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 476/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk476<F: Float>(t1917: F, t1940: F, t1830: F, t1833: F, t1845: F) -> (F, F, F) {
    let t1941 = t1917 * t1940;
    let t1944 = F::new(0.12361111111111111111e-1) * t1830;
    let t1947 = t1944 - F::new(0.18541666666666666667e-1) * t1833 + F::new(0.278125e-1) * t1845;
    (t1941, t1944, t1947)
}
