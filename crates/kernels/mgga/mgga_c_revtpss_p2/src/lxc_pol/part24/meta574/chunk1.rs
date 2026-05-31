//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1758/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1758<F: Float>(t1132: F, t90450: F, t3407: F, t90419: F, t141: F, t3417: F, t89841: F, t89826: F, t81230: F, t81232: F, t81234: F, t81425: F, t81427: F, t81429: F, t89828: F, t89843: F, t89847: F, t89855: F) -> (F, F, F, F, F) {
    let t90459 = t1132 * t90450;
    let t90464 = t3407 * t90419;
    let t90470 = t141 * t3417 * t89841;
    let t90473 = t141 * t3417 * t89826;
    let t90478 = -F::cast_from(0.72462e1_f64) * t89828 + F::cast_from(0.258925e1_f64) * t90459 - F::cast_from(0.22076e0_f64) * t81425 + F::cast_from(0.44152e0_f64) * t81427 - F::cast_from(0.132456e1_f64) * t81429 + F::cast_from(0.247573125e0_f64) * t90464 - F::cast_from(0.80513333333333333332e0_f64) * t89843 + F::cast_from(0.108693e2_f64) * t89847 + F::cast_from(0.24154e1_f64) * t89855 - F::cast_from(0.11038e0_f64) * t90470 - F::cast_from(0.99342e0_f64) * t90473 - F::cast_from(0.44729629629629629629e0_f64) * t81230 + F::cast_from(0.16102666666666666667e1_f64) * t81232 - F::cast_from(0.24154e1_f64) * t81234;
    (t90459, t90464, t90470, t90473, t90478)
}
