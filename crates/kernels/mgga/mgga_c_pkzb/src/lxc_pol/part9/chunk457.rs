//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 457/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk457<F: Float>(t1861: F, t1862: F, t1830: F, t1833: F, t1845: F) -> (F, F, F) {
    let t1863 = t1861 * t1862;
    let t1865 = F::new(4.0) / F::new(9.0) * t1830;
    let t1867 = t1865 - F::new(2.0) / F::new(3.0) * t1833 + t1845;
    (t1863, t1865, t1867)
}
