//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 446/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk446<F: Float>(t1917: F, t24: F, t712: F, t1243: F, t1251: F, t248: F, t256: F, t528: F, t713: F, t1881: F, t1884: F, t1890: F, t1895: F, t1900: F, t1902: F, t1905: F, t1907: F, t1911: F, t1915: F) -> (F, F, F, F) {
    let t1918 = t24 * t1917;
    let t1920 = F::cast_from(0.12155555555555555555e0_f64) * t712 * t1918;
    let t1923 = -F::cast_from(0.43111111111111111111e-1_f64) * t1243 + F::cast_from(0.18777777777777777778e0_f64) * t1251;
    let t1924 = t248 * t1923;
    let t1926 = t1924 * t256 / F::new(3.0);
    let t1928 = F::cast_from(0.33245444444444444444e-1_f64) * t528 * t713;
    let t1929 = t1881 + t1884 - t1890 - t1895 - t1900 + t1902 - t1905 + t1907 * t256 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1911 + F::cast_from(0.12155555555555555555e0_f64) * t1915 + t1920 + t1926 + t1928;
    (t1918, t1923, t1924, t1929)
}
