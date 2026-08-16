//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 975/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk975(t7845: f64, t3454: f64, t572: f64, t418: f64, t5548: f64, t587: f64, t1017: f64, t995: f64, t610: f64, t7703: f64, t1820: f64, t2585: f64, t7130: f64) -> (f64, f64, f64, f64) {
    let t11004 = 8.0_f64 / 135.0_f64 * t7845;
    let t11005 = t3454 * t572;
    let t11006 = t11005 * t418;
    let t11007 = t5548 * t11006;
    let t11009 = 8.0_f64 / 45.0_f64 * t587 * t11007;
    let t11010 = t995 * t1017;
    let t11011 = t11010 * t610;
    let t11012 = t7703 * t11011;
    let t11014 = 16.0_f64 / 15.0_f64 * t1820 * t11012;
    let t11016 = 16.0_f64 / 45.0_f64 * t7130 * t2585;
    (t11004, t11009, t11014, t11016)
}
