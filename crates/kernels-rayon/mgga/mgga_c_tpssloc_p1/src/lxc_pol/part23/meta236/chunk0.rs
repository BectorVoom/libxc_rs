//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 887/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk887(t5576: f64, t838: f64, t5631: f64, t814: f64, t252: f64, t5611: f64, t1499: f64, t4280: f64, t225: f64, t5559: f64, t5632: f64, t5561: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17000 = t5576 * t838;
    let t17027 = t814 * t5631;
    let t17030 = t252 * t5611;
    let t17034 = t1499 * t4280;
    let t17052 = t5559 * t225;
    let t17090 = t5632 * t225;
    let t17092 = t5561 * t225;
    (t17000, t17027, t17030, t17034, t17052, t17090, t17092)
}
