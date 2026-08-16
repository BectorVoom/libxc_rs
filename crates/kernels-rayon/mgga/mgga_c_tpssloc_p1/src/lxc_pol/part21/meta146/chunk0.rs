//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 950/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk950(t3475: f64, t457: f64, t460: f64, t974: f64, t1174: f64, t3430: f64, t3433: f64, t3436: f64, t3443: f64, t3447: f64, t3452: f64, t3457: f64, t3461: f64, t3472: f64) -> (f64, f64, f64) {
    let t3477 = t457 * t3475 * t460;
    let t3478 = t974 * t3477;
    let t3481 = -t3430 - 0.18518518518518518518e-3_f64 * t3433 - 0.55555555555555555554e-3_f64 * t3436 + 0.37037037037037037036e-3_f64 * t1174 * t3443 + 0.55555555555555555554e-3_f64 * t3447 * t3452 - 0.55555555555555555554e-3_f64 * t1174 * t3457 - 0.27777777777777777777e-3_f64 * t1174 * t3461 - 0.83333333333333333332e-3_f64 * t1174 * t3472 - 0.83333333333333333332e-3_f64 * t1174 * t3478;
    (t3477, t3478, t3481)
}
