//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1401/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401<F: Float>(t39494: F, t3964: F, t4096: F, t2453: F, t9679: F, t3906: F, t3907: F, t1359: F, t39501: F, t10115: F, t555: F, t123: F, t125: F, t1358: F, t8779: F, t9645: F) -> (F, F, F, F, F, F) {
    let t47454 = F::cast_from(0.20561456923286030469e-1_f64) * t3964 * t4096 * t39494;
    let t47480 = t2453 * t9679;
    let t47504 = F::cast_from(0.20561456923286030469e-1_f64) * t3906 * t3907 * t39494;
    let t47561 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1359;
    let t47567 = t10115 * t555;
    let t47591 = F::cast_from(0.65457331274007190912e-5_f64) * t123 * t125 * t8779 * t9645 * t555 * t1358;
    (t47454, t47480, t47504, t47561, t47567, t47591)
}
