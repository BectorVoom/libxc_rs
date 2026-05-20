//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1979/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979<F: Float>(t25386: F, t92837: F, t25372: F, t2435: F, t25352: F, t11015: F, t7018: F, t7048: F, t822: F, t25300: F, t9285: F, t25299: F) -> (F, F, F, F, F, F, F) {
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92858 = t2435 * t25352;
    let t92861 = F::cast_from(0.30356481678079769392e-1_f64) * t7018 * t11015;
    let t92864 = t822 * t7048;
    let t92868 = t25300 * t9285;
    let t92870 = F::cast_from(0.68540937416128198417e-2_f64) * t25299 * t92868;
    (t92838, t92843, t92858, t92861, t92864, t92868, t92870)
}
