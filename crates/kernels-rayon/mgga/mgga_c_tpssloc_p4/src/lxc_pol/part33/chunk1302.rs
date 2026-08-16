//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1302/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1302(t23518: f64, t5928: f64, t23384: f64, t28657: f64, t1920: f64, t28630: f64, t968: f64, t28618: f64, t28671: f64, t82736: f64, t28610: f64, t28557: f64, t6743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100240 = t23518 * t5928;
    let t100254 = t23384 * t28657;
    let t100324 = t1920 * t968 * t28630;
    let t100378 = t23384 * t28618;
    let t100390 = t82736 * t28671;
    let t100399 = t23384 * t28610;
    let t100417 = t28557 * t6743;
    (t100240, t100254, t100324, t100378, t100390, t100399, t100417)
}
