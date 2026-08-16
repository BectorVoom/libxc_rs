//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1849/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849<F: Float>(t25386: F, t95536: F, t26518: F, t9285: F, t25299: F, t2061: F, t22: F, t25402: F, t93140: F, t25310: F, t26506: F, t2439: F, t7398: F, t780: F, t785: F) -> (F, F, F, F, F, F, F) {
    let t95537 = t25386 * t95536;
    let t95540 = t26518 * t9285;
    let t95542 = F::cast_from(0.68540937416128198417e-2_f64) * t25299 * t95540;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = F::cast_from(0.51727911450665971904e-3_f64) * t93140 * t95546;
    let t95551 = t25310 * t26506;
    let t95562 = t2439 * t785 * t7398 * t780;
    (t95537, t95540, t95542, t95546, t95548, t95551, t95562)
}
