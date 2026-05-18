//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 982/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk982<F: Float>(t10115: F, t557: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t1398: F, t215: F, t268: F, t543: F, t4101: F) -> (F, F, F, F) {
    let t10117 = F::new(0.11044544084478153697e-3) * t10115 * t557;
    let t10126 = F::new(0.17073386770573548589e-1) * t9292 * t1429;
    let t10129 = F::new(0.46263278077393568556e-2) * t3964 * t4096 * t9285;
    let t10136 = t268 * t215 * t1398 * t543;
    let t10137 = t4101 * t10136;
    (t10117, t10126, t10129, t10137)
}
