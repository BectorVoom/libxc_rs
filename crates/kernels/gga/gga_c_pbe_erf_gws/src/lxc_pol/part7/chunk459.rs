//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 459/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk459<F: Float>(t226: F, t678: F, t230: F, t666: F, t1826: F, t1831: F, t1870: F, t1874: F, t1876: F, t1881: F, t1884: F, t1890: F, t1895: F, t1900: F) -> F {
    let t2014 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t226 * t678;
    let t2015 = t666 * t230;
    let t2017 = t1826 - t1831 - t1870 + t1874 - t1876 + t1881 + t1884 + t2014 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2015 - t1890 - t1895 - t1900;
    t2017
}
