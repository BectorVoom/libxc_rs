//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 444/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk444<F: Float>(t1903: F, t252: F, t1354: F, t247: F, t251: F, t707: F, t719: F, t256: F, t19: F, t535: F, t336: F) -> (F, F, F, F, F, F, F) {
    let t1905 = F::new(2.0) / F::new(27.0) * t252 * t1903;
    let t1906 = t1354 * t247;
    let t1907 = t1906 * t251;
    let t1910 = t707 * t719;
    let t1911 = t1910 * t256;
    let t1913 = t535 * t19;
    let t1914 = t1913 * t336;
    (t1905, t1906, t1907, t1910, t1911, t1913, t1914)
}
