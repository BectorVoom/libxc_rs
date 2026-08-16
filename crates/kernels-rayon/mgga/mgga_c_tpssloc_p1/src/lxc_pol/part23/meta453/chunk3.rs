//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1308/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308(t39316: f64, t39320: f64, t39373: f64, t39397: f64, t39400: f64, t40679: f64, t40685: f64, t40708: f64, t75854: f64, t75855: f64, t75856: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40714: f64, t40716: f64, t40721: f64, t75864: f64, t75865: f64) -> (f64, f64) {
    let t76007 = t39316 + t39320 - t40679 - t40685 + t75854 - t75855 + t75856 + t39373 - t39397 - t39400 + t40708;
    let t76009 = t39408 + t39411 - t40714 + t40716 + t75864 - t75865 + t39463 - t39468 - t40721 - t39472 - t39476;
    (t76007, t76009)
}
