//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 468/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk468<F: Float>(t1403: F, t1896: F, t590: F, t587: F, t720: F, t723: F, t156: F, t254: F, t252: F, t1354: F, t247: F, t251: F) -> (F, F, F, F, F, F, F, F) {
    let t1897 = t1896 * t1403;
    let t1898 = t590 * t1897;
    let t1900 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t1898;
    let t1902 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t720 * t723;
    let t1903 = t254 * t156;
    let t1905 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t252 * t1903;
    let t1906 = t1354 * t247;
    let t1907 = t1906 * t251;
    (t1897, t1898, t1900, t1902, t1903, t1905, t1906, t1907)
}
