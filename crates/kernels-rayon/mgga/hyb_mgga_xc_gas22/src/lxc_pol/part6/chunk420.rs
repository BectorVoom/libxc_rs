//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 420/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk420(t109: f64, t1849: f64, t1572: f64, t310: f64, t884: f64, t647: f64, t664: f64, t105: f64, t121: f64, t1828: f64, t1834: f64, t1838: f64, t1840: f64, t1842: f64, t1846: f64, t651: f64, t656: f64, t660: f64, t96: f64) -> (f64, f64, f64, f64) {
    let t1850 = t109 * t1849;
    let t1851 = t1572 * t310;
    let t1854 = t884 * t1849;
    let t1855 = t1854 * t1572;
    let t1858 = t664 * t647;
    let t1861 = 0.37552696856994557333e-1_f64 * t96 * t1828 * t105 - 0.35400808369803607838e-3_f64 * t651 * t1834 * t656 + 0.80569443951744882604e-6_f64 * t1838 * t1840 * t1842 - 40.0_f64 / 9.0_f64 * t660 * t1846 + 50.0_f64 / 9.0_f64 * t1850 * t1851 + 50.0_f64 / 9.0_f64 * t121 * t1855 - 40.0_f64 / 9.0_f64 * t121 * t1858;
    (t1851, t1855, t1858, t1861)
}
