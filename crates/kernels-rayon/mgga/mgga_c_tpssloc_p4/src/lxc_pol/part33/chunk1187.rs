//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1187/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1187(t343: f64, t5836: f64, t6734: f64, t5842: f64, t1941: f64, t5904: f64, t1011: f64, t5872: f64, t3131: f64, t23512: f64, t360: f64, t23519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28557 = t5836 * t343;
    let t28558 = t28557 * t6734;
    let t28565 = t5842 * t343;
    let t28566 = t28565 * t6734;
    let t28572 = t5904 * t1941;
    let t28576 = t5872 * t1011;
    let t28577 = t28576 * t3131;
    let t28578 = t23512 * t28577;
    let t28581 = t28576 * t360;
    let t28582 = t23519 * t28581;
    (t28557, t28558, t28565, t28566, t28572, t28577, t28578, t28581, t28582)
}
