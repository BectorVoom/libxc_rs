//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 967/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk967<F: Float>(t8887: F, t8899: F, t829: F, t2229: F, t3316: F, t1333: F, t2233: F) -> (F, F, F, F) {
    let t8900 = t8887 + t8899;
    let t8901 = t8900 * t829;
    let t8905 = F::new(1.0) * t3316 * t2229;
    let t8906 = t1333 * t2233;
    (t8900, t8901, t8905, t8906)
}
