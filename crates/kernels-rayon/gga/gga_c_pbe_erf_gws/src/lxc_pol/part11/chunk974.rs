//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 974/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk974(t1243: f64, t3426: f64, t1251: f64, t3437: f64, t3430: f64, t3434: f64, t3440: f64, t3422: f64, t1033: f64, t7844: f64, t10418: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31643 = t1243 * t3426;
    let t31777 = t1251 * t3437;
    let t31785 = t1243 * t3430;
    let t31801 = t1251 * t3434;
    let t31803 = t1251 * t3440;
    let t31805 = t1243 * t3422;
    let t31879 = t1033 * t7844;
    let t32019 = t10418 * t586;
    (t31643, t31777, t31785, t31801, t31803, t31805, t31879, t32019)
}
