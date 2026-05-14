//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1123/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1123<F: Float>(t1858: F, t3040: F, t579: F, t1879: F, t6134: F, t1863: F, t1870: F, t1866: F, t17: F, t8223: F) -> (F, F, F, F, F, F, F) {
    let t21353 = t1858 * t1858;
    let t21354 = 1.0 / t21353;
    let t21365 = t3040 * t579;
    let t21367 = t6134 * t1879;
    let t21369 = t6134 * t1863;
    let t21373 = t6134 * t1870;
    let t21397 = 1.0 / t1858 / t1866;
    let t21425 = t8223 * t17;
    (t21354, t21365, t21367, t21369, t21373, t21397, t21425)
}
