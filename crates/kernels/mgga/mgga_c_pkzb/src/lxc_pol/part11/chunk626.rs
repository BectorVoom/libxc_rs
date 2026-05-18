//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 626/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk626<F: Float>(t204: F, t205: F, t3515: F, t1831: F, t2730: F, t228: F, t1084: F, t2746: F, t1083: F, t684: F) -> (F, F, F, F, F, F) {
    let t3517 = t204 * t205 * t3515;
    let t3519 = t1831 - F::new(0.35616666666666666666e-1) * t2730 + F::new(0.53425e-1) * t3517;
    let t3521 = F::new(0.621814e-1) * t3519 * t228;
    let t3523 = F::new(2.0) * t2746 * t1084;
    let t3524 = t1083 * t1083;
    let t3525 = t3524 * t684;
    (t3517, t3519, t3521, t3523, t3524, t3525)
}
