//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 825/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk825(t1409: f64, t22510: f64, t24498: f64, t27356: f64, t5392: f64, t5398: f64, t5415: f64, t56: f64, t7251: f64, t67: f64, t1864: f64, t7445: f64, t7974: f64) -> (f64, f64, f64) {
    let t29473 = 88.0_f64 / 9.0_f64 * t5415 * t56 + 40.0_f64 / 9.0_f64 * t27356 * t1409 + 5.0_f64 / 18.0_f64 * t24498 * t5392 - 5.0_f64 / 6.0_f64 * t7251 * t5398 - t22510;
    let t29474 = t29473 * t67;
    let t29475 = t29474 * t1864;
    let t29478 = t7974 * t7445;
    (t29473, t29475, t29478)
}
