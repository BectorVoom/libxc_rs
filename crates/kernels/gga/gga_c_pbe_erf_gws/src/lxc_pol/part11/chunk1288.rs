//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1288/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1288<F: Float>(t1105: F, t1115: F, t1161: F, t12138: F, t12213: F, t13086: F, t13212: F, t13606: F, t13612: F, t13648: F, t2118: F, t2376: F, t2408: F, t2409: F, t3066: F, t3067: F, t3079: F, t328: F, t335: F, t3373: F, t338: F, t339: F, t36152: F, t376: F, t3912: F, t3917: F, t46641: F, t46650: F, t46678: F, t46685: F, t46976: F, t49245: F, t49259: F, t49273: F, t49279: F, t49281: F, t49283: F, t49285: F, t49295: F, t49299: F, t49315: F, t49828: F, t49832: F, t49837: F, t49839: F, t49845: F, t49852: F, t49857: F, t49859: F, t49861: F, t49875: F, t50565: F, t50567: F, t50568: F, t50571: F, t50572: F, t50574: F, t50575: F, t50580: F, t50582: F, t50583: F, t50586: F, t50587: F, t50589: F, t50590: F, t9296: F, t9849: F) -> F {
    let t50617 = t3912 * t2118 * t3373 * t328 * t3079 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t46650 - t1115 * t46641 / F::cast_from(8.0_f64) - t3917 * t12138 / F::cast_from(4.0_f64) + t3066 * t2409 * t3067 * t1161 * t13606 / F::cast_from(12.0_f64) - t9849 * t13212 / F::cast_from(12.0_f64) + t1115 * t46976 / F::cast_from(4.0_f64) + t335 * t338 * t339 * (-t49828 + t49852 - t49279 - t49315 - t49245 + t49259 + t49837 + t49273 + t49299 - t49832 + t49295 + t50589 + t50590 - t49845 - t49857 - t49859 - t49861 + t50580 + t50582 + t50583 + t50565 + t49875 - t49281 - t49283 + t50586 + t50587 + t50567 + t50568 + t50571 + t49285 - t49839 + t50572 + t50574 + t50575) * t376 / F::cast_from(96.0_f64) + t2408 * t2409 * t9296 * t13612 * t1105 / F::cast_from(2.0_f64) - t2408 * t2409 * t12213 * t13648 / F::cast_from(2.0_f64) + t2408 * t2409 * t2376 * t13086 * t1161 / F::cast_from(12.0_f64) - F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t36152 - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t46678 + F::cast_from(7.0_f64) / F::cast_from(6.0_f64) * t46685;
    t50617
}
