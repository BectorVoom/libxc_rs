//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1301/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1301(t23384: f64, t28663: f64, t23511: f64, t5928: f64, t28638: f64, t23665: f64, t28605: f64, t5932: f64, t6743: f64, t28653: f64, t82822: f64, t5936: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100163 = t23384 * t28663;
    let t100165 = t23511 * t5928;
    let t100189 = t23384 * t28638;
    let t100193 = t23665 * t28605;
    let t100204 = t6743 * t5932;
    let t100215 = t82822 * t28653;
    let t100231 = t6743 * t5936;
    (t100163, t100165, t100189, t100193, t100204, t100215, t100231)
}
