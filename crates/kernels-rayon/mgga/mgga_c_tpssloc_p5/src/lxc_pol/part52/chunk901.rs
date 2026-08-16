//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 901/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk901(t2020: f64, t8690: f64, t113: f64, t1869: f64, t1976: f64, t2114: f64, t2165: f64, t510: f64, t574: f64, t8315: f64, t8322: f64, t8324: f64, t8329: f64, t8451: f64, t8491: f64, t8495: f64, t8667: f64, t8669: f64, t8676: f64, t8682: f64, t8687: f64) -> f64 {
    let t8691 = t8690 * t2020;
    let t8692 = -t113 * t8682 - t1869 * t2165 - t1976 * t2114 - t510 * t8667 + t574 * t8687 - 2.0_f64 * t8315 - t8322 - 2.0_f64 * t8324 - t8329 + t8451 + t8491 - t8495 - 2.0_f64 * t8669 - 2.0_f64 * t8676 + t8691;
    t8692
}
