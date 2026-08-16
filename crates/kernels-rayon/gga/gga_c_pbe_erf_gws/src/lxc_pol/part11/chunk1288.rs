//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1288/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1288(t1105: f64, t1115: f64, t1161: f64, t12138: f64, t12213: f64, t13086: f64, t13212: f64, t13606: f64, t13612: f64, t13648: f64, t2118: f64, t2376: f64, t2408: f64, t2409: f64, t3066: f64, t3067: f64, t3079: f64, t328: f64, t335: f64, t3373: f64, t338: f64, t339: f64, t36152: f64, t376: f64, t3912: f64, t3917: f64, t46641: f64, t46650: f64, t46678: f64, t46685: f64, t46976: f64, t49245: f64, t49259: f64, t49273: f64, t49279: f64, t49281: f64, t49283: f64, t49285: f64, t49295: f64, t49299: f64, t49315: f64, t49828: f64, t49832: f64, t49837: f64, t49839: f64, t49845: f64, t49852: f64, t49857: f64, t49859: f64, t49861: f64, t49875: f64, t50565: f64, t50567: f64, t50568: f64, t50571: f64, t50572: f64, t50574: f64, t50575: f64, t50580: f64, t50582: f64, t50583: f64, t50586: f64, t50587: f64, t50589: f64, t50590: f64, t9296: f64, t9849: f64) -> f64 {
    let t50617 = t3912 * t2118 * t3373 * t328 * t3079 / 24.0_f64 - 7.0_f64 / 24.0_f64 * t46650 - t1115 * t46641 / 8.0_f64 - t3917 * t12138 / 4.0_f64 + t3066 * t2409 * t3067 * t1161 * t13606 / 12.0_f64 - t9849 * t13212 / 12.0_f64 + t1115 * t46976 / 4.0_f64 + t335 * t338 * t339 * (-t49828 + t49852 - t49279 - t49315 - t49245 + t49259 + t49837 + t49273 + t49299 - t49832 + t49295 + t50589 + t50590 - t49845 - t49857 - t49859 - t49861 + t50580 + t50582 + t50583 + t50565 + t49875 - t49281 - t49283 + t50586 + t50587 + t50567 + t50568 + t50571 + t49285 - t49839 + t50572 + t50574 + t50575) * t376 / 96.0_f64 + t2408 * t2409 * t9296 * t13612 * t1105 / 2.0_f64 - t2408 * t2409 * t12213 * t13648 / 2.0_f64 + t2408 * t2409 * t2376 * t13086 * t1161 / 12.0_f64 - 35.0_f64 / 36.0_f64 * t36152 - 7.0_f64 / 4.0_f64 * t46678 + 7.0_f64 / 6.0_f64 * t46685;
    t50617
}
