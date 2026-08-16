//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1323/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1323(t41209: f64, t41212: f64, t46806: f64, t59195: f64, t59204: f64, t59206: f64, t59218: f64, t59221: f64, t59224: f64, t68116: f64, t68118: f64, t68122: f64, t68131: f64) -> f64 {
    let t76371 = 0.11111111111111111111e-2_f64 * t46806 - 0.77777777777777777775e-1_f64 * t59195 + 0.15555555555555555555e-1_f64 * t68116 + 0.18666666666666666665e0_f64 * t68118 + 0.39999999999999999998e-1_f64 * t68122 + 0.33333333333333333332e-2_f64 * t68131 + t41209 + t41212 + 0.23333333333333333332e0_f64 * t59204 + 0.94999999999999999997e-1_f64 * t59206 - 0.31666666666666666666e-1_f64 * t59218 - 0.29999999999999999998e-1_f64 * t59221 + 0.99999999999999999996e-2_f64 * t59224;
    t76371
}
