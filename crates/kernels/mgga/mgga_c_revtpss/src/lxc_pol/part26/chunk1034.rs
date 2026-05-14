//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1034/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1034<F: Float>(t2470: F, t26543: F, t7058: F, t122: F, t25412: F, t72: F, t7398: F, t25431: F, t2646: F, t26481: F, t676: F, t26482: F, t93374: F, t7385: F, t9292: F, t2772: F, t689: F, t7384: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95575 = t26543 * t2470;
    let t95576 = t7058 * t95575;
    let t95593 = t7398 * t72 * t122 * t25412;
    let t95594 = t25431 * t95593;
    let t95597 = t26481 * t676 * t2646;
    let t95598 = t25431 * t95597;
    let t95604 = t93374 * t26482;
    let t95607 = 0.17073386770573548589e-1 * t9292 * t7385;
    let t95613 = t689 * t7384 * t2772;
    (t95575, t95576, t95593, t95594, t95597, t95598, t95604, t95607, t95613)
}
