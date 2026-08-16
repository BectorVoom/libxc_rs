//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 464/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk464<F: Float>(t1866: F, t598: F, t186: F, t185: F, t1406: F, t198: F, t561: F, t579: F, t612: F, t1789: F, t1797: F, t1800: F, t1808: F, t1814: F, t1819: F, t1826: F, t1831: F, t1841: F, t267: F) -> (F, F, F, F, F, F, F, F) {
    let t1867 = t598 * t1866;
    let t1868 = t186 * t1867;
    let t1870 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t1868;
    let t1871 = t198 * t1406;
    let t1872 = t186 * t1871;
    let t1874 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t561 * t1872;
    let t1876 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t579 * t612;
    let t1877 = t1789 + t1797 + t1800 + t1808 + t1814 - t1819 + t1826 - t1831 - t1841 * t267 / F::cast_from(15.0_f64) - t1870 + t1874 - t1876;
    (t1867, t1868, t1870, t1871, t1872, t1874, t1876, t1877)
}
