//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 448/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk448<F: Float>(t19: F, t535: F, t336: F, t714: F, t247: F, t719: F, t24: F, t712: F, t1243: F, t1251: F, t248: F, t256: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1913 = t535 * t19;
    let t1914 = t1913 * t336;
    let t1915 = t1914 * t714;
    let t1917 = t247 * t719;
    let t1918 = t24 * t1917;
    let t1920 = F::cast_from(0.12155555555555555555e0_f64) * t712 * t1918;
    let t1923 = -F::cast_from(0.43111111111111111111e-1_f64) * t1243 + F::cast_from(0.18777777777777777778e0_f64) * t1251;
    let t1924 = t248 * t1923;
    let t1926 = t1924 * t256 / F::cast_from(3.0_f64);
    (t1913, t1914, t1915, t1917, t1918, t1920, t1923, t1924, t1926)
}
