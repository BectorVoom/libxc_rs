//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 419/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk419(t894: f64, t95: f64, t318: f64, t97: f64, t104: f64, t655: f64, t123: f64, t647: f64, tau0: f64) -> (f64, f64, f64, f64, f64) {
    let t1838 = t95 * t894;
    let t1839 = t318 * t97;
    let t1840 = 1.0_f64 / t1839;
    let t1842 = 1.0_f64 / t655 / t104;
    let t1846 = t647 * t123;
    let t1849 = tau0 * tau0;
    (t1838, t1840, t1842, t1846, t1849)
}
