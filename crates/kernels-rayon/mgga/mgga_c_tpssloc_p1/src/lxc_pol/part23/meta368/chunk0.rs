//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1168/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1168(t11147: f64, t460: f64, t11545: f64, t135: f64, t43791: f64, t461: f64, t3439: f64, t698: f64, t1176: f64, t697: f64, t11153: f64, t3242: f64, t405: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44505 = t460 * t11147;
    let t44562 = t135 * t11545;
    let t44566 = t461 * t43791;
    let t44571 = t698 * t3439;
    let t44583 = t697 * t1176;
    let t44607 = t460 * t11153;
    let t44620 = 1.0_f64 / t405 / t3242;
    (t44505, t44562, t44566, t44571, t44583, t44607, t44620)
}
