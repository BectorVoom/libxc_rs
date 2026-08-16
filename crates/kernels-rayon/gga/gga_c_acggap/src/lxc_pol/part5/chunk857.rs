//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 857/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk857(t12200: f64, t464: f64, t181: f64, t862: f64, t322: f64, t3888: f64, t448: f64, t3868: f64, t3915: f64, t1220: f64, t1221: f64, t316: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t12201 = t12200 * t464;
    let t12203 = t862 * t181;
    let t12206 = t12203 * t448 * t322 * t3888;
    let t12208 = t3868 * t3915;
    let t12212 = t316 * t1220 * t879 * t1221;
    (t12201, t12203, t12206, t12208, t12212)
}
