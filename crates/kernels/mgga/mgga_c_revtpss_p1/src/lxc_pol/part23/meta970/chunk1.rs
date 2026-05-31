//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3271/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3271<F: Float>(t1353: F, t13768: F, t13902: F, t13910: F, t1392: F, t1394: F, t1412: F, t1877: F, t21969: F, t22236: F, t22249: F, t22287: F, t22809: F, t22813: F, t22944: F, t22947: F, t22950: F, t539: F, t5591: F, t5650: F, t5651: F, t6816: F, t85442: F, t9940: F) -> F {
    let t86052 = -F::cast_from(12.0_f64) * t1353 * t1412 * t22809 * t5650 - F::cast_from(360.0_f64) * t1353 * t22813 * t5650 * t9940 + F::cast_from(180.0_f64) * t13768 * t22287 * t5650 - F::cast_from(36.0_f64) * t13910 * t5650 * t6816 + F::cast_from(3.0_f64) * t1394 * t539 * t85442 - F::cast_from(36.0_f64) * t21969 * t5650 * t5651 + F::cast_from(180.0_f64) * t22236 * t5591 * t5650 - F::cast_from(36.0_f64) * t13902 * t22947 + F::cast_from(60.0_f64) * t1392 * t22944 + F::cast_from(3.0_f64) * t1392 * t22950 + F::cast_from(9.0_f64) * t1877 * t22249;
    t86052
}
