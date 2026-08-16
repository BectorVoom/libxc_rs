//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 676/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk676(t2515: f64, t414: f64, t1336: f64, t960: f64, t1396: f64, t2840: f64, t1392: f64, t1218: f64, t242: f64, t3013: f64, t2519: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8012 = t414 * t2515;
    let t8014 = t1336 * t960;
    let t8016 = t2840 * t1396;
    let t8018 = t2840 * t1392;
    let t8023 = t2840 * t1218;
    let t8042 = t3013 * t242;
    let t8051 = t2519 * t700;
    (t8012, t8014, t8016, t8018, t8023, t8042, t8051)
}
