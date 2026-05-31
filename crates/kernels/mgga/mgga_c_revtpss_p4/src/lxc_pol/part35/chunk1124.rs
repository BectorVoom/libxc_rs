//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1124/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1124<F: Float>(t93140: F, t95546: F, t93134: F, t26435: F, t9303: F, t7385: F, t9292: F, t11015: F, t7388: F, t92975: F, t92988: F, t92995: F) -> (F, F, F, F, F, F, F, F) {
    let t95548 = F::cast_from(0.51727911450665971904e-3_f64) * t93140 * t95546;
    let t95567 = F::cast_from(0.43639970290213137151e-3_f64) * t93134 * t95546;
    let t95569 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t26435;
    let t95607 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7385;
    let t95632 = F::cast_from(0.30356481678079769392e-1_f64) * t7388 * t11015;
    let t95666 = F::cast_from(0.18295201011342718161e-3_f64) * t92975;
    let t95671 = F::cast_from(0.3252886739816735289e-3_f64) * t92988;
    let t95673 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t92995;
    (t95548, t95567, t95569, t95607, t95632, t95666, t95671, t95673)
}
