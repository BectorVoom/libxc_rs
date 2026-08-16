//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2652/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652(t1409: f64, t628: f64, t67: f64, t2250: f64, t5398: f64, t16558: f64, t607: f64, t12606: f64, t12620: f64, t12623: f64, t12662: f64, t12665: f64, t1411: f64, t1434: f64, t17635: f64, t1864: f64, t19322: f64, t19323: f64, t19363: f64, t19404: f64, t2251: f64, t3966: f64, t3968: f64, t3971: f64, t4018: f64, t5427: f64, t608: f64, t642: f64, t65: f64, t6509: f64, t80: f64) -> (f64, f64, f64) {
    let t55653 = t1409 * t628 * t67;
    let t55662 = t2250 * t5398;
    let t55666 = t607 * t16558;
    let t55673 = -t12662 * t1434 / 6.0_f64 - t12665 * t1434 / 3.0_f64 - t3968 * t4018 / 3.0_f64 - t12623 * t1434 / 6.0_f64 - t3971 * t4018 / 3.0_f64 - t1411 * t12620 / 6.0_f64 - t2251 * t5427 * t80 / 12.0_f64 - t608 * t19404 * t80 / 6.0_f64 - t19363 * t642 / 6.0_f64 - t55653 * t19323 / 3.0_f64 - t19322 * t6509 * t3966 / 3.0_f64 - t19322 * t1864 * t12606 / 6.0_f64 - t55662 * t65 * t80 / 12.0_f64 - t55666 * t65 * t80 / 6.0_f64 - t17635 * t628 * t80 / 6.0_f64;
    (t55662, t55666, t55673)
}
