//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1365/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1365(t105731: f64, t25927: f64, t20947: f64, t25891: f64, t1649: f64, t5660: f64, t105762: f64, t23788: f64, t5664: f64, t28248: f64, t89992: f64, t1530: f64, t5966: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106671 = t25927 * t105731;
    let t106677 = t25891 * t20947;
    let t106686 = t1649 * t5660;
    let t106690 = t23788 * t105762;
    let t106699 = t1649 * t5664;
    let t106706 = t89992 * t28248;
    let t106712 = t5966 * t1530;
    (t106671, t106677, t106686, t106690, t106699, t106706, t106712)
}
