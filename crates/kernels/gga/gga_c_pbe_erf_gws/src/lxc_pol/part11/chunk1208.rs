//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1208/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1208<F: Float>(t3780: F, t2079: F, t1115: F, t1161: F, t12111: F, t12234: F, t13127: F, t13212: F, t13688: F, t20154: F, t2376: F, t2383: F, t3047: F, t3052: F, t326: F, t3913: F, t3917: F, t43304: F, t43328: F, t43344: F, t43357: F, t46925: F, t833: F, t8629: F, t9885: F, t9902: F) -> (F, F, F) {
    let t48997 = t3780 * t3780;
    let t48998 = t2079 * t48997;
    let t49019 = -t13688 * t3047 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t43304 - t9902 * t13212 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t43328 + t3913 * t12234 / F::cast_from(16.0_f64) + t326 * t48998 * t2383 * t833 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t43344 - t8629 * t20154 * t2376 * t43357 * t1161 / F::cast_from(4.0_f64) - t13127 * t3052 / F::cast_from(12.0_f64) - t13127 * t3047 / F::cast_from(24.0_f64) - t3913 * t9885 / F::cast_from(8.0_f64) - t1115 * t46925 / F::cast_from(4.0_f64) + t3917 * t12111 / F::cast_from(8.0_f64);
    (t48997, t48998, t49019)
}
