//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2348/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2348(t16662: f64, t221: f64, t4127: f64, t4128: f64, t46790: f64, t46794: f64, t46796: f64, t46806: f64, t46856: f64, t59195: f64, t68110: f64, t68116: f64, t68118: f64, t68122: f64) -> f64 {
    let t68124 = 0.14999999999999999999e-1_f64 * t4127 * t221 * t4128 * t16662 - 0.74999999999999999995e-2_f64 * t68110 + 0.16851851851851851851e0_f64 * t46790 + t46794 + 0.47499999999999999999e-1_f64 * t46796 + 0.8333333333333333333e-3_f64 * t46806 - 0.38888888888888888887e-1_f64 * t59195 - t46856 + 0.38888888888888888887e-2_f64 * t68116 + 0.46666666666666666664e-1_f64 * t68118 + 0.99999999999999999995e-2_f64 * t68122;
    t68124
}
