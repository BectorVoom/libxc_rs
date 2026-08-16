//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2221/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2221<F: Float>(t108879: F, t2122: F, t101237: F, t101240: F, t101243: F, t104215: F, t104226: F, t108872: F, t108876: F, t108941: F, t108945: F, t1923: F, t2123: F, t26792: F, t28154: F, t29380: F, t29532: F, t30689: F, t6954: F, t7575: F, t92568: F, t96804: F) -> F {
    let t111639 = t2122 * t108879;
    let t111652 = -t6954 * t30689 / F::cast_from(6.0_f64) - t1923 * t7575 * t29532 / F::cast_from(6.0_f64) - t1923 * t2122 * t108941 / F::cast_from(6.0_f64) + t108945 * t2123 / F::cast_from(3.0_f64) + F::cast_from(35.0_f64) * t96804 * t108872 - F::cast_from(10.0_f64) * t26792 * t108876 + F::cast_from(10.0_f64) * t92568 * t111639 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101237 * t29380 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101240 * t29380 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t101243 * t29380 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t104215 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t104226;
    t111652
}
