//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1199/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1199(t23754: f64, t3275: f64, t3465: f64, t11325: f64, t11555: f64, t3582: f64, t38718: f64, t3579: f64, t38749: f64, t11559: f64, t11189: f64, t3262: f64, t40620: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41308 = t3275 * t3465 * t23754 / 4.0_f64;
    let t41311 = 5.0_f64 / 8.0_f64 * t3275 * t11325 * t11555;
    let t41314 = 5.0_f64 / 16.0_f64 * t3275 * t38718 * t3582;
    let t41316 = 5.0_f64 / 16.0_f64 * t3579 * t38749;
    let t41319 = 5.0_f64 / 8.0_f64 * t3275 * t11325 * t11559;
    let t41322 = 135.0_f64 / 64.0_f64 * t3262 * t11189 * t40620;
    (t41308, t41311, t41314, t41316, t41319, t41322)
}
