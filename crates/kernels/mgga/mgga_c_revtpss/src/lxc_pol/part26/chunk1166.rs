//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1166/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1166<F: Float>(t198: F, t206: F, t7427: F, t2411: F, t26580: F, t25373: F, t26550: F, t25386: F, t92840: F, t26518: F, t9285: F, t25299: F) -> (F, F, F, F, F, F) {
    let t95511 = t198 * t206 * t7427;
    let t95527 = t26580 * t2411;
    let t95536 = t25373 * t26550;
    let t95537 = t25386 * t95536;
    let t95538 = t95537 * t92840;
    let t95540 = t26518 * t9285;
    let t95542 = F::new(0.68540937416128198417e-2) * t25299 * t95540;
    (t95511, t95527, t95536, t95538, t95540, t95542)
}
