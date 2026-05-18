//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 968/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk968<F: Float>(t2237: F, t8906: F, t1347: F, t6569: F, t3396: F, t839: F, t1363: F, t2311: F) -> (F, F, F, F) {
    let t8908 = F::new(0.16081979498692535067e2) * t8906 * t2237;
    let t8910 = F::new(1.0) * t6569 * t1347;
    let t8911 = t3396 * t839;
    let t8916 = t1363 * t2311;
    (t8908, t8910, t8911, t8916)
}
