//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1406/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1406(t50834: f64, t71335: f64, t71337: f64, t77959: f64, t77963: f64, t77967: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t77989: f64, t77992: f64, t77995: f64, t77998: f64) -> f64 {
    let t78000 = 0.44152e0_f64 * t77959 - 0.8585111111111111111e-1_f64 * t77963 - 0.82785e-1_f64 * t77967 + 0.49671e0_f64 * t77971 - 0.99342e0_f64 * t77975 + 0.198684e1_f64 * t77979 + 0.82785e-1_f64 * t77983 + 0.22076e0_f64 * t71335 - 0.132456e1_f64 * t71337 - 0.12524296296296296297e1_f64 * t50834 + 0.72462e1_f64 * t77989 + 0.301925e0_f64 * t77992 - 0.89459259259259259259e0_f64 * t77995 + 0.181155e1_f64 * t77998;
    t78000
}
