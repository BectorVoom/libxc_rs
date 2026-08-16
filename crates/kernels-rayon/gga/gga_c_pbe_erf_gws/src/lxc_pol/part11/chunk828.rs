//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 828/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk828(t1076: f64, t3776: f64, t3373: f64, t11318: f64, t12381: f64, t13156: f64, t2107: f64, t3030: f64, t323: f64, t6096: f64, t818: f64, t9150: f64) -> (f64, f64, f64) {
    let t13164 = t3776 * t1076;
    let t13167 = t1076 * t3373;
    let t13171 = -3.0_f64 * t1076 * t11318 - t12381 * t818 + t13156 * t323 - 6.0_f64 * t13164 * t6096 + 6.0_f64 * t13167 * t2107 - 3.0_f64 * t3030 * t3373 + 6.0_f64 * t3776 * t9150;
    (t13164, t13167, t13171)
}
