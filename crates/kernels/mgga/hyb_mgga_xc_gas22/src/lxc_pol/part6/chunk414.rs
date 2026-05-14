//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 414/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk414<F: Float>(t109: F, t1849: F, t1572: F, t310: F, t884: F, t647: F, t664: F, t105: F, t121: F, t1828: F, t1834: F, t1838: F, t1840: F, t1842: F, t1846: F, t651: F, t656: F, t660: F, t96: F) -> (F, F, F, F) {
    let t1850 = t109 * t1849;
    let t1851 = t1572 * t310;
    let t1854 = t884 * t1849;
    let t1855 = t1854 * t1572;
    let t1858 = t664 * t647;
    let t1861 = 0.37552696856994557333e-1 * t96 * t1828 * t105 - 0.35400808369803607838e-3 * t651 * t1834 * t656 + 0.80569443951744882604e-6 * t1838 * t1840 * t1842 - 40.0 / 9.0 * t660 * t1846 + 50.0 / 9.0 * t1850 * t1851 + 50.0 / 9.0 * t121 * t1855 - 40.0 / 9.0 * t121 * t1858;
    (t1851, t1855, t1858, t1861)
}
