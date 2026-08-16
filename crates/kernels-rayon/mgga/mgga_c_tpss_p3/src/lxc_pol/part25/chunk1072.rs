//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1072/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1072(t14632: f64, t904: f64, t4886: f64, t876: f64, t10980: f64, t11309: f64, t11312: f64, t14495: f64, t14497: f64, t14501: f64, t14503: f64, t14505: f64, t14507: f64, t8616: f64, t8627: f64) -> (f64, f64, f64) {
    let t14734 = t14632 * t904;
    let t14739 = t4886 * t876;
    let t14770 = 0.11477222222222222222e0_f64 * t14495 + 0.23154444444444444445e-1_f64 * t14497 - 0.22954444444444444444e0_f64 * t8616 - 0.11577222222222222222e0_f64 * t8627 - 0.13892666666666666667e0_f64 * t14501 + 0.69463333333333333333e-1_f64 * t14503 - 0.34431666666666666667e0_f64 * t14505 + 0.17215833333333333333e0_f64 * t14507 - 0.45908888888888888888e0_f64 * t10980 + t11309 + t11312;
    (t14734, t14739, t14770)
}
