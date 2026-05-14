//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1129/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1129<F: Float>(t1118: F, t1144: F, t13110: F, t13639: F, t13678: F, t2401: F, t2409: F, t3067: F, t3207: F, t335: F, t338: F, t36290: F, t3703: F, t3721: F, t3737: F, t3887: F, t3896: F, t3907: F, t3917: F, t4386: F, t43983: F, t46759: F, t46858: F, t46870: F, t46872: F, t844: F, t8629: F, t8793: F, t9899: F) -> (F,) {
    let t50681 = t8629 * t4386 * t1144 * t13110 / 4.0 + t8793 * t43983 / 4.0 - 7.0 / 24.0 * t46759 + 3.0 / 8.0 * t2401 * t338 * t3907 * t3737 - t844 * t338 * t13678 * t1118 / 12.0 - t844 * t338 * t1144 * t13639 / 12.0 - t844 * t338 * t3907 * t3896 / 8.0 - 35.0 / 72.0 * t36290 - t3917 * t9899 / 16.0 - 7.0 / 72.0 * t46858 - 7.0 / 24.0 * t46870 + 3.0 / 4.0 * t3207 * t2409 * t3067 * t3703 * t3721 - 7.0 / 6.0 * t46872 - t335 * t338 * t3907 * t3887 / 16.0;
    (t50681,)
}
