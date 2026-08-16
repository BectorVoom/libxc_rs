//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 682/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk682(t1378: f64, t1971: f64, t8361: f64, t1049: f64, t1986: f64, t2007: f64, t2970: f64, t2000: f64, t20: f64, t2653: f64, t2004: f64, t678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8390 = t8361 * t1378 * t1971;
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    let t8414 = t2970 * t2000;
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    let t8440 = t1049 * t678;
    (t8390, t8405, t8408, t8414, t8424, t8425, t8440)
}
