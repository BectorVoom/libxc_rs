//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1209/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1209<F: Float>(t2453: F, t4086: F, t64: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t25877: F, t94390: F, t7285: F, t9288: F) -> (F, F, F, F, F) {
    let t94564 = t2453 * t4086 * t64;
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = F::new(0.22589491248727328397e-6) * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = F::new(0.14450132032386466905e-2) * t94570;
    let t94589 = t94390 * t25877;
    let t94600 = t7285 * t9288;
    (t94564, t94569, t94571, t94589, t94600)
}
