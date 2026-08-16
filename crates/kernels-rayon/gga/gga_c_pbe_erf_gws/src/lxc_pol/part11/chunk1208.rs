//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1208/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1208(t3780: f64, t2079: f64, t1115: f64, t1161: f64, t12111: f64, t12234: f64, t13127: f64, t13212: f64, t13688: f64, t20154: f64, t2376: f64, t2383: f64, t3047: f64, t3052: f64, t326: f64, t3913: f64, t3917: f64, t43304: f64, t43328: f64, t43344: f64, t43357: f64, t46925: f64, t833: f64, t8629: f64, t9885: f64, t9902: f64) -> (f64, f64, f64) {
    let t48997 = t3780 * t3780;
    let t48998 = t2079 * t48997;
    let t49019 = -t13688 * t3047 / 12.0_f64 + 7.0_f64 / 72.0_f64 * t43304 - t9902 * t13212 / 8.0_f64 + 7.0_f64 / 24.0_f64 * t43328 + t3913 * t12234 / 16.0_f64 + t326 * t48998 * t2383 * t833 / 32.0_f64 - 7.0_f64 / 24.0_f64 * t43344 - t8629 * t20154 * t2376 * t43357 * t1161 / 4.0_f64 - t13127 * t3052 / 12.0_f64 - t13127 * t3047 / 24.0_f64 - t3913 * t9885 / 8.0_f64 - t1115 * t46925 / 4.0_f64 + t3917 * t12111 / 8.0_f64;
    (t48997, t48998, t49019)
}
