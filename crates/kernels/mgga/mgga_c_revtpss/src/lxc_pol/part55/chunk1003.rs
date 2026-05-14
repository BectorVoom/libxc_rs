//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1003/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1003<F: Float>(t119776: F, t31830: F, t2487: F, t31753: F, t8478: F, t8479: F, t2769: F, t32425: F, t2670: F, t31827: F, t31809: F, t31845: F, t11007: F, t3140: F, t822: F, t1032: F, t7063: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119788 = t31830 * t119776;
    let t119789 = t119788 * t2487;
    let t119790 = 0.13223814266738539448e-3 * t119789;
    let t119792 = t8478 * t8479 * t31753;
    let t119808 = t32425 * t2769;
    let t119816 = t31827 * t2670;
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119833 = t7063 * t1032;
    (t119788, t119790, t119792, t119808, t119816, t119818, t119821, t119822, t119823, t119833)
}
