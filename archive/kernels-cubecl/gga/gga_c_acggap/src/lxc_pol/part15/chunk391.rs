//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 391/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk391<F: Float>(t1866: F, t960: F, t1514: F, t1516: F, t1542: F, t1565: F, t1817: F, t1841: F, t1846: F, t1851: F, t1856: F, t1861: F, t335: F, t397: F, t418: F, t942: F) -> (F, F) {
    let t1867 = t960 * t1866;
    let t1872 = F::cast_from(0.42874018118069736972e-3_f64) * t942 * t1817 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t1841 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t1846 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1851 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t1856 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1861 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t1514 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t1516 + t335 * t1867 / F::cast_from(24.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t1542 - F::cast_from(0.17149607247227894789e-2_f64) * t1565;
    (t1867, t1872)
}
