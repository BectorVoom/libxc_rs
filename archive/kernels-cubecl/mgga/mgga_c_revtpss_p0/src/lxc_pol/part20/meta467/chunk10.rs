//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1796/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1796<F: Float>(t46250: F, t47692: F, t10259: F, t116: F, t117: F, t13232: F, t13240: F, t13243: F, t13244: F, t13247: F, t1459: F, t1461: F, t2327: F, t2371: F, t4158: F, t4162: F, t4165: F, t46137: F, t46233: F, t572: F, t573: F, param_d: F) -> (F, F) {
    let t47693 = t46250 + t47692;
    let t47728 = F::cast_from(24.0_f64) * t10259 * t13243 * t572 + F::cast_from(18.0_f64) * t116 * t46137 * t572 + F::cast_from(3.0_f64) * t117 * t46233 * t572 + F::cast_from(36.0_f64) * t2327 * t2371 * t572 + t47693 * t573 * param_d + F::cast_from(12.0_f64) * t13232 * t1461 + F::cast_from(24.0_f64) * t13240 * t1459 + F::cast_from(72.0_f64) * t13244 * t1459 + F::cast_from(12.0_f64) * t13247 * t1459 + F::cast_from(36.0_f64) * t4158 * t4162 + F::cast_from(18.0_f64) * t4158 * t4165;
    (t47693, t47728)
}
