//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3180/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3180<F: Float>(t1222: F, t17240: F, t24244: F, t20982: F, t20986: F, t21126: F, t21129: F, t21239: F, t5312: F, t5373: F, t5391: F, t57480: F, t57491: F, t70733: F, t81173: F, t81182: F, t81212: F) -> F {
    let t83504 = t1222 * t17240 * t24244;
    let t83526 = t57491 - t83504 / F::cast_from(144.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t70733 + F::cast_from(7.0_f64) / F::cast_from(81.0_f64) * t5373 * t21129 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1222 * t57480 * t81212 - t5373 * t21126 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5373 * t21239 + t1222 * t5312 * t81182 / F::cast_from(216.0_f64) + t1222 * t5312 * t81173 / F::cast_from(6.0_f64) + F::cast_from(0.91464571985215438872e-2_f64) * t5391 * t20982 + F::cast_from(0.13719685797782315831e-1_f64) * t5391 * t20986;
    t83526
}
