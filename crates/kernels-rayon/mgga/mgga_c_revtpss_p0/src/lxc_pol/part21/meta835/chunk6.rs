//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3134/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3134(t12832: f64, t17620: f64, t17412: f64, t3636: f64, t1196: f64, t12500: f64, t16672: f64, t12227: f64, t1732: f64, t1149: f64, t12230: f64, t3427: f64) -> (f64, f64, f64, f64) {
    let t57780 = t12832 * t17620;
    let t57786 = t17412 * t3636;
    let t57794 = 0.51947577317044391277e2_f64 * t1196 * t16672 * t12500;
    let t57795 = t12227 * t1732;
    let t57799 = 0.1551780387578202009e4_f64 * t57795 * t12230 * t3427 * t1149;
    (t57780, t57786, t57794, t57799)
}
