//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2461/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2461<F: Float>(t1427: F, t1444: F, t22: F, t9647: F, t123: F, t125: F, t1358: F, t555: F, t8779: F, t9645: F, t268: F, t39644: F, t556: F, t561: F) -> (F, F, F) {
    let t47574 = t9647 * t1427 * t22 * t1444;
    let t47591 = F::cast_from(0.65457331274007190912e-5_f64) * t123 * t125 * t8779 * t9645 * t555 * t1358;
    let t47601 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t556 * t561 * t8779 * t268;
    (t47574, t47591, t47601)
}
