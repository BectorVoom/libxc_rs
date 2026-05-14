//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 455/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk455<F: Float>(t1406: F, t198: F, t186: F, t561: F, t579: F, t612: F, t1789: F, t1797: F, t1800: F, t1808: F, t1814: F, t1819: F, t1826: F, t1831: F, t1841: F, t1870: F, t267: F) -> (F, F, F, F, F) {
    let t1871 = t198 * t1406;
    let t1872 = t186 * t1871;
    let t1874 = 4.0 / 15.0 * t561 * t1872;
    let t1876 = 4.0 / 15.0 * t579 * t612;
    let t1877 = t1789 + t1797 + t1800 + t1808 + t1814 - t1819 + t1826 - t1831 - t1841 * t267 / 15.0 - t1870 + t1874 - t1876;
    (t1871, t1872, t1874, t1876, t1877)
}
