//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2059/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2059<F: Float>(t25978: F, t4014: F, t25972: F, t9923: F, t2453: F, t4086: F, t64: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F) -> (F, F, F, F, F, F) {
    let t94559 = t25978 * t4014;
    let t94561 = t25972 * t9923;
    let t94564 = t2453 * t4086 * t64;
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = F::cast_from(0.22589491248727328397e-6_f64) * t94568;
    let t94570 = t9784 * t7256;
    (t94559, t94561, t94564, t94565, t94569, t94570)
}
