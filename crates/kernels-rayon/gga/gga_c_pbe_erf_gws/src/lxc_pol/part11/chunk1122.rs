//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1122/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1122(t17646: f64, t3390: f64, t3465: f64, t639: f64, t1640: f64, t47377: f64, t5401: f64, t1661: f64, t47391: f64, t5294: f64, t587: f64, t10843: f64, t3504: f64) -> (f64, f64, f64, f64) {
    let t47878 = 16.0_f64 / 9.0_f64 * t639 * t17646 * t3465 * t3390;
    let t47882 = 16.0_f64 / 3.0_f64 * t639 * t1640 * t5401 * t47377;
    let t47886 = 16.0_f64 / 3.0_f64 * t587 * t1661 * t5294 * t47391;
    let t47888 = 32.0_f64 / 15.0_f64 * t10843 * t3504;
    (t47878, t47882, t47886, t47888)
}
