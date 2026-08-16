//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1376/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1376(t42212: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64) -> f64 {
    let t77287 = -0.125034e1_f64 * t76893 + 0.55570666666666666666e0_f64 * t76896 + 0.250068e1_f64 * t76909 + 0.62517e0_f64 * t76915 - 0.15302962962962962963e1_f64 * t76574 - 0.516475e0_f64 * t76578 + 0.68863333333333333334e1_f64 * t76583 - 0.123954e2_f64 * t76591 - 0.103295e1_f64 * t76599 + 0.123954e2_f64 * t76614 + 0.309885e1_f64 * t76622 + 0.27545333333333333333e1_f64 * t59688 - 0.13772666666666666666e1_f64 * t59694 + t42212;
    t77287
}
