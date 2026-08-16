//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1421/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1421(t1164: f64, t4861: f64, t72062: f64, t5988: f64, t11277: f64, t43969: f64, t50834: f64, t71335: f64, t71337: f64, t77959: f64, t77963: f64, t77967: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t77989: f64, t77992: f64, t77995: f64, t77998: f64) -> (f64, f64, f64, f64) {
    let t78128 = 0.69263436422725855036e2_f64 * t1164 * t72062 * t4861;
    let t78129 = t5988 * t5988;
    let t78132 = 0.62071215503128080361e4_f64 * t43969 * t78129 * t11277;
    let t78147 = 0.43816888888888888889e0_f64 * t77959 - 0.85199506172839506175e-1_f64 * t77963 - 0.82156666666666666668e-1_f64 * t77967 + 0.49293999999999999999e0_f64 * t77971 - 0.98587999999999999998e0_f64 * t77975 + 0.197176e1_f64 * t77979 + 0.82156666666666666667e-1_f64 * t77983 + 0.21908444444444444444e0_f64 * t71335 - 0.13145066666666666666e1_f64 * t71337 - 0.12401580246913580247e1_f64 * t50834 + 0.71752e1_f64 * t77989 + 0.29896666666666666667e0_f64 * t77992 - 0.88582716049382716048e0_f64 * t77995 + 0.17938e1_f64 * t77998;
    (t78128, t78129, t78132, t78147)
}
