//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3135/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135(t12248: f64, t16661: f64, t3385: f64, t12357: f64, t1733: f64, t3384: f64, t12228: f64, t12592: f64, t5192: f64, t1765: f64, t45319: f64, t12411: f64, t17092: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57802 = 0.28947563097646563121e3_f64 * t12248 * t16661 * t3385;
    let t57805 = 2.0_f64 * t3384 * t1733 * t12357;
    let t57808 = 24.0_f64 * t12248 * t1733 * t12228;
    let t57810 = 0.10254018858216406658e4_f64 * t5192 * t12592;
    let t57812 = 0.5848223622634646207e0_f64 * t45319 * t1765;
    let t57814 = 6.0_f64 * t17092 * t12411;
    (t57802, t57805, t57808, t57810, t57812, t57814)
}
