//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 797/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk797<F: Float>(t1161: F, t3886: F, t2409: F, t3067: F, t1144: F, t338: F, t3887: F, t1118: F, t3907: F, t12187: F, t13207: F, t13212: F, t13217: F, t13223: F, t13229: F, t13609: F, t13615: F, t13619: F, t2408: F, t3055: F, t3066: F, t335: F, t3733: F, t6731: F, t6816: F, t844: F, t8818: F, t9275: F, t9290: F, t9902: F) -> (F, F, F, F, F) {
    let t13622 = t1161 * t3886;
    let t13624 = t2409 * t3067 * t13622;
    let t13628 = t338 * t1144 * t3887;
    let t13635 = t338 * t3907 * t1118;
    let t13638 = -t9902 * t3733 / 32.0 + t2408 * t13207 / 16.0 - t3055 * t13212 / 32.0 - t3055 * t13217 / 96.0 - t6816 * t13223 / 4.0 - 35.0 / 144.0 * t8818 + t2408 * t13229 / 16.0 - t335 * t13609 / 96.0 - t335 * t13615 / 16.0 + t335 * t13619 / 16.0 + t3066 * t13624 / 16.0 - t335 * t13628 / 32.0 - t6731 - 7.0 / 16.0 * t12187 + 35.0 / 144.0 * t9275 - 35.0 / 72.0 * t9290 - t844 * t13635 / 16.0;
    (t13622, t13624, t13628, t13635, t13638)
}
