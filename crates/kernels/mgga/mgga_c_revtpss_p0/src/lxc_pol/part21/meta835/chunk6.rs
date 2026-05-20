//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3134/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134<F: Float>(t12832: F, t17620: F, t17412: F, t3636: F, t1196: F, t12500: F, t16672: F, t12227: F, t1732: F, t1149: F, t12230: F, t3427: F) -> (F, F, F, F) {
    let t57780 = t12832 * t17620;
    let t57786 = t17412 * t3636;
    let t57794 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t16672 * t12500;
    let t57795 = t12227 * t1732;
    let t57799 = F::cast_from(0.1551780387578202009e4_f64) * t57795 * t12230 * t3427 * t1149;
    (t57780, t57786, t57794, t57799)
}
