//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1862/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862<F: Float>(t25386: F, t95536: F, t92840: F, t26518: F, t9285: F, t25299: F, t7407: F, t92890: F, t2061: F, t22: F, t25402: F, t93140: F) -> (F, F, F, F, F, F, F) {
    let t95537 = t25386 * t95536;
    let t95538 = t95537 * t92840;
    let t95540 = t26518 * t9285;
    let t95542 = F::cast_from(0.68540937416128198417e-2_f64) * t25299 * t95540;
    let t95543 = t92890 * t7407;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = F::cast_from(0.51727911450665971904e-3_f64) * t93140 * t95546;
    (t95537, t95538, t95540, t95542, t95543, t95546, t95548)
}
