//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1922/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1922<F: Float>(t1479: F, t60: F, t25137: F, t26776: F, t4181: F, t4186: F, t606: F, t7571: F, t72: F, t1927: F, t6977: F, t8143: F) -> (F, F, F, F, F) {
    let t29355 = t1479 * t60;
    let t29362 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t29355 * t606 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t26776 * t4181 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7571 * t4186 - t25137;
    let t29363 = t29362 * t72;
    let t29364 = t29363 * t1927;
    let t29367 = t8143 * t6977;
    (t29355, t29362, t29363, t29364, t29367)
}
