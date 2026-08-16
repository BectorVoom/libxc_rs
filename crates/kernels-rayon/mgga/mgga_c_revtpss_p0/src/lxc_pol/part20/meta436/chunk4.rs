//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1647/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647(t1149: f64, t12357: f64, t3433: f64, t3435: f64, t12227: f64, t12230: f64, t3385: f64, t3427: f64, t3386: f64, t1130: f64, t12393: f64, t1151: f64) -> (f64, f64, f64, f64) {
    let t45033 = 0.64327917994770140268e2_f64 * t3433 * t12357 * t3435 * t1149;
    let t45037 = 0.3103560775156404018e4_f64 * t12227 * t3385 * t12230 * t3427;
    let t45040 = 36.0_f64 * t3433 * t3386 * t3427;
    let t45041 = t12393 * t1130;
    let t45043 = 4.0_f64 * t45041 * t1151;
    (t45033, t45037, t45040, t45043)
}
