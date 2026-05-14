//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1005/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1005<F: Float>(t94973: F, t25373: F, t26550: F, t25386: F, t26518: F, t9285: F, t25299: F, t2061: F, t22: F, t25402: F, t93140: F, t93134: F, t26435: F, t9303: F, t7385: F, t9292: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95397 = 308.0 / 27.0 * t94973;
    let t95536 = t25373 * t26550;
    let t95537 = t25386 * t95536;
    let t95540 = t26518 * t9285;
    let t95542 = 0.68540937416128198417e-2 * t25299 * t95540;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = 0.51727911450665971904e-3 * t93140 * t95546;
    let t95567 = 0.43639970290213137151e-3 * t93134 * t95546;
    let t95569 = 0.26019841438354088051e-2 * t9303 * t26435;
    let t95607 = 0.17073386770573548589e-1 * t9292 * t7385;
    (t95397, t95536, t95537, t95540, t95542, t95548, t95567, t95569, t95607)
}
