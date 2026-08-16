//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1213/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1213(t843: f64, t1962: f64, t41154: f64, t25373: f64, t25392: f64, t25386: f64, t25372: f64, t11015: f64, t7018: f64, t25300: f64, t9285: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92612 = 1232.0_f64 / 27.0_f64 * t843;
    let t92742 = t1962 * t41154;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92861 = 0.30356481678079769392e-1_f64 * t7018 * t11015;
    let t92868 = t25300 * t9285;
    let t92870 = 0.68540937416128198417e-2_f64 * t25299 * t92868;
    (t92612, t92742, t92838, t92843, t92861, t92868, t92870)
}
