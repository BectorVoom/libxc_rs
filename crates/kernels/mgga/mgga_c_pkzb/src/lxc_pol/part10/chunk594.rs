//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 594/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk594<F: Float>(t2369: F, t2371: F, t758: F, t2099: F, t922: F, t918: F, t178: F, t916: F, t915: F) -> (F, F, F, F, F) {
    let t2372 = t2369 * t2371;
    let t2373 = t758 * t2372;
    let t2376 = t2099 * t922;
    let t2377 = t918 * t2376;
    let t2379 = t916 * t178;
    let t2380 = t915 * t2379;
    (t2372, t2373, t2376, t2377, t2380)
}
