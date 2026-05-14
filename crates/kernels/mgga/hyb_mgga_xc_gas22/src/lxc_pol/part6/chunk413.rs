//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 413/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk413<F: Float>(t894: F, t95: F, t318: F, t97: F, t104: F, t655: F, t123: F, t647: F, tau0: F) -> (F, F, F, F, F) {
    let t1838 = t95 * t894;
    let t1839 = t318 * t97;
    let t1840 = 1.0 / t1839;
    let t1842 = 1.0 / t655 / t104;
    let t1846 = t647 * t123;
    let t1849 = tau0 * tau0;
    (t1838, t1840, t1842, t1846, t1849)
}
