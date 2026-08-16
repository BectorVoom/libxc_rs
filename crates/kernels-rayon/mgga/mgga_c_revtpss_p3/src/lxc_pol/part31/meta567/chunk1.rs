//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1979/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979(t25386: f64, t92837: f64, t25372: f64, t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92858 = t2435 * t25352;
    let t92861 = 0.30356481678079769392e-1_f64 * t7018 * t11015;
    let t92864 = t822 * t7048;
    let t92868 = t25300 * t9285;
    let t92870 = 0.68540937416128198417e-2_f64 * t25299 * t92868;
    (t92838, t92843, t92858, t92861, t92864, t92868, t92870)
}
