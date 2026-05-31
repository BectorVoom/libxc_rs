//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1358/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1358<F: Float>(t2371: F, t28264: F, t572: F, t2327: F, t7002: F, t1459: F, t26120: F, t26124: F, t26127: F, t13240: F, t13244: F, t13247: F, t1461: F, t2040: F, t26106: F, t4162: F, t4165: F, t573: F, t7324: F, t95119: F, t95131: F, t95136: F, t95140: F, t95143: F, t95147: F, t95149: F, t95153: F, t95157: F, param_d: F) -> F {
    let t95160 = F::cast_from(18.0_f64) * t572 * t28264 * t2371;
    let t95163 = F::cast_from(18.0_f64) * t572 * t2327 * t7002;
    let t95171 = F::cast_from(18.0_f64) * t1459 * t26120;
    let t95173 = F::cast_from(36.0_f64) * t1459 * t26124;
    let t95175 = F::cast_from(18.0_f64) * t1459 * t26127;
    let t95176 = t573 * t95119 * param_d + F::cast_from(6.0_f64) * t13240 * t2040 + F::cast_from(18.0_f64) * t13244 * t2040 + F::cast_from(3.0_f64) * t13247 * t2040 + F::cast_from(9.0_f64) * t1461 * t26106 + F::cast_from(18.0_f64) * t4162 * t7324 + F::cast_from(9.0_f64) * t4165 * t7324 + t95131 + t95136 + t95140 + t95143 + t95147 + t95149 + t95153 + t95157 + t95160 + t95163 + t95171 + t95173 + t95175;
    t95176
}
