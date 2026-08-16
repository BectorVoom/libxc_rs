//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1004/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1004(t1528: f64, t259: f64, t4147: f64, t4268: f64, t5559: f64, t5561: f64, t5632: f64, t5637: f64, t5658: f64, t855: f64) -> f64 {
    let t5660 = -2.0_f64 * t1528 * t4147 - 2.0_f64 * t1528 * t4268 + t259 * t5559 + 2.0_f64 * t259 * t5561 + t259 * t5632 + 2.0_f64 * t5637 * t855 - t5658 * t855;
    t5660
}
