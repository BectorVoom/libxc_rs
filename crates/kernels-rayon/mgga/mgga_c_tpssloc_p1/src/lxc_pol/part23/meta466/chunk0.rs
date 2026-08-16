//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1364/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364(t42086: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64) -> f64 {
    let t77097 = -0.98587999999999999998e0_f64 * t76893 + 0.43816888888888888889e0_f64 * t76896 + 0.197176e1_f64 * t76909 + 0.49293999999999999999e0_f64 * t76915 - 0.88582716049382716048e0_f64 * t76574 - 0.29896666666666666667e0_f64 * t76578 + 0.39862222222222222223e1_f64 * t76583 - 0.71752000000000000002e1_f64 * t76591 - 0.59793333333333333333e0_f64 * t76599 + 0.71752e1_f64 * t76614 + 0.17938e1_f64 * t76622 + 0.15944888888888888889e1_f64 * t59688 - 0.79724444444444444446e0_f64 * t59694 + t42086;
    t77097
}
