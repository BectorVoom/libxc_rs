//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 392/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk392(t1089: f64, t405: f64, t974: f64, t3242: f64, t461: f64, t337: f64, t51: f64, t1887: f64, t1176: f64, t60: f64, t460: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3439 = 1.0_f64 / t405 / t1089;
    let t3440 = t974 * t3439;
    let t3441 = t461 * t3242;
    let t3446 = t51 * t337;
    let t3447 = t3446 * t1887;
    let t3448 = t60 * t1176;
    let t3450 = t460 * t1089;
    let t3455 = t461 * t3247;
    (t3439, t3440, t3441, t3447, t3448, t3450, t3455)
}
