//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1093/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1093<F: Float>(t1844: F, t301: F, t1181: F, t599: F, t7337: F, t368: F, t5659: F, t7380: F, t1795: F, t355: F, t1083: F, t2095: F) -> (F, F, F, F, F, F, F) {
    let t38883 = t1844 * t301;
    let t38886 = t7337 * t1181 * t599 * t38883;
    let t38889 = t368 * t5659;
    let t38890 = t7380 * t38889;
    let t38892 = t355 * t1795;
    let t38893 = t1083 * t38892;
    let t38894 = t2095 * t38893;
    (t38883, t38886, t38889, t38890, t38892, t38893, t38894)
}
