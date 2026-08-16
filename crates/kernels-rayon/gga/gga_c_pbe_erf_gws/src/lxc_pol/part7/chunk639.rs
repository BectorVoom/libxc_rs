//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 639/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk639(t219: f64, t5002: f64, t4367: f64, t1640: f64, t639: f64, t197: f64, t4957: f64, t4352: f64, t1661: f64, t587: f64, t1866: f64, t562: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5003 = t219 * t5002;
    let t5004 = t5003 * t4367;
    let t5005 = t1640 * t5004;
    let t5007 = 8.0_f64 / 9.0_f64 * t639 * t5005;
    let t5008 = t197 * t4957;
    let t5009 = t5008 * t4352;
    let t5010 = t1661 * t5009;
    let t5012 = 8.0_f64 / 9.0_f64 * t587 * t5010;
    let t5014 = t597 * t1866 * t562;
    (t5003, t5004, t5005, t5007, t5008, t5009, t5010, t5012, t5014)
}
