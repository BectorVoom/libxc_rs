//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1496/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496(t54428: f64, t193: f64, t20416: f64, t3918: f64, t3924: f64, t39490: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39518: f64, t39521: f64, t39529: f64, t39539: f64, t39549: f64, t39563: f64, t5122: f64, t79921: f64) -> (f64, f64) {
    let t79925 = 144.0_f64 * t54428;
    let t79926 = 18.0_f64 * t193 * t3924 * t79921 + 12.0_f64 * t20416 * t3918 * t5122 - t39490 - t39496 + t39499 + t39502 - t39505 - t39508 + t39518 - t39521 - t39529 + t39539 + t39549 + t39563 + t79925;
    (t79925, t79926)
}
