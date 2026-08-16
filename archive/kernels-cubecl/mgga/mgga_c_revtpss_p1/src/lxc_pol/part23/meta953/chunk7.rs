//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3170/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3170<F: Float>(t1012: F, t1222: F, t1225: F, t17649: F, t17654: F, t20767: F, t20938: F, t21111: F, t21119: F, t21210: F, t5373: F, t5381: F, t57094: F, t70278: F, t70281: F, t70300: F, t70306: F, t70990: F, t71440: F, t76397: F, t83033: F) -> F {
    let t83281 = -F::cast_from(0.19055119163586549765e-3_f64) * t70278 - F::cast_from(0.1270341277572436651e-2_f64) * t70281 - F::cast_from(0.19055119163586549765e-2_f64) * t5381 * t21111 - F::cast_from(0.85748036236139473944e-3_f64) * t70300 + t5373 * t21210 / F::cast_from(36.0_f64) - t1222 * t1012 * t1225 * t76397 / F::cast_from(288.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t70306 + F::cast_from(0.95275595817932748827e-4_f64) * t57094 + F::cast_from(0.91464571985215438872e-2_f64) * t70990 * t20767 + F::cast_from(0.91464571985215438872e-2_f64) * t71440 * t20938 - F::cast_from(0.85748036236139473944e-3_f64) * t17654 * t17649 * t83033 * t21119;
    t83281
}
