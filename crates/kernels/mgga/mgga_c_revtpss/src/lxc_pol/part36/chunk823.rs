//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 823/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk823<F: Float>(t10115: F, t557: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t2453: F, t4100: F, t562: F, t64: F, t843: F) -> (F, F, F, F, F, F) {
    let t10117 = F::new(0.11044544084478153697e-3) * t10115 * t557;
    let t10126 = F::new(0.17073386770573548589e-1) * t9292 * t1429;
    let t10129 = F::new(0.46263278077393568556e-2) * t3964 * t4096 * t9285;
    let t10139 = t2453 * t4100;
    let t10157 = F::new(0.11044544084478153697e-3) * t10115 * t562;
    let t10199 = t64 * t843;
    (t10117, t10126, t10129, t10139, t10157, t10199)
}
