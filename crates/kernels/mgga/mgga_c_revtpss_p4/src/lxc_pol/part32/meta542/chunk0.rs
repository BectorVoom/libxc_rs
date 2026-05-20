//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1853/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1853<F: Float>(t7399: F, t786: F, t867: F, t93173: F, t95725: F, t93371: F, t2453: F, t26496: F, t10506: F, t10510: F, t26497: F, t10073: F, t25402: F, t7056: F, t7398: F) -> (F, F, F, F, F, F, F) {
    let t95743 = t786 * t7399 * t867;
    let t95746 = t95725 * t93173;
    let t95747 = t93371 * t95746;
    let t95773 = t2453 * t26496;
    let t95774 = t95773 * t10506;
    let t95779 = t26497 * t10510;
    let t95783 = t10073 * t7056 * t25402 * t7398;
    (t95743, t95746, t95747, t95773, t95774, t95779, t95783)
}
