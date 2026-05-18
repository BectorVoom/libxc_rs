//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 484/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk484<F: Float>(t513: F, t530: F, t960: F, t1514: F, t1516: F, t1542: F, t1565: F, t1817: F, t1841: F, t1846: F, t1851: F, t1856: F, t1861: F, t335: F, t397: F, t418: F, t942: F) -> (F, F, F) {
    let t1866 = t530 * t513;
    let t1867 = t960 * t1866;
    let t1872 = F::new(0.42874018118069736972e-3) * t942 * t1817 - F::new(0.21437009059034868486e-3) * t397 * t1841 - F::new(0.21437009059034868486e-3) * t397 * t1846 + F::new(0.34299214494455789578e-2) * t418 * t1851 - F::new(0.17149607247227894789e-2) * t418 * t1856 - F::new(0.34299214494455789578e-2) * t418 * t1861 + F::new(7.0) / F::new(144.0) * t1514 + F::new(7.0) / F::new(72.0) * t1516 + t335 * t1867 / F::new(24.0) + F::new(0.42874018118069736972e-3) * t1542 - F::new(0.17149607247227894789e-2) * t1565;
    (t1866, t1867, t1872)
}
