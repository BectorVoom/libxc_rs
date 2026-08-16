//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1074/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1074<F: Float>(t3390: F, t6442: F, t3394: F, t5044: F, t6423: F, t6427: F, t6431: F, t1132: F, t3407: F, t1139: F, t3417: F, t6421: F) -> (F, F, F, F, F, F) {
    let t6443 = t3390 * t6442;
    let t6449 = t3394 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5044 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6423 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6427 + t6431 / F::cast_from(3.0_f64);
    let t6450 = t1132 * t6449;
    let t6456 = t3407 * t6442;
    let t6458 = t1139 * t6449;
    let t6461 = t3417 * t6421;
    (t6443, t6449, t6450, t6456, t6458, t6461)
}
