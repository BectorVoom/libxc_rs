//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1865/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1865<F: Float>(t25431: F, t95593: F, t2646: F, t26481: F, t676: F, t26482: F, t93374: F, t7385: F, t9292: F, t2772: F, t689: F, t7384: F) -> (F, F, F, F, F, F) {
    let t95594 = t25431 * t95593;
    let t95597 = t26481 * t676 * t2646;
    let t95598 = t25431 * t95597;
    let t95604 = t93374 * t26482;
    let t95607 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7385;
    let t95613 = t689 * t7384 * t2772;
    (t95594, t95597, t95598, t95604, t95607, t95613)
}
