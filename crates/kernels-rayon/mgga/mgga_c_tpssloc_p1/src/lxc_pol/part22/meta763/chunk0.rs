//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2570/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2570(t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64, t71461: f64, t71463: f64, t71465: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64) -> (f64, f64) {
    let t71955 = -0.103295e1_f64 * t71183 - 0.103295e1_f64 * t71187 + 0.31558125e0_f64 * t71446 - 0.17648625e1_f64 * t71449 - 0.6618234375e1_f64 * t71452 + 0.794188125e1_f64 * t71454 - 0.52945875e1_f64 * t71456 - 0.52945875e1_f64 * t71458 + 0.2366859375e0_f64 * t71461 - 0.473371875e0_f64 * t71463 + 0.94674375e0_f64 * t71465;
    let t71968 = 0.94674375e0_f64 * t71468 - 0.30872592592592592593e-1_f64 * t71470 + 0.13892666666666666667e0_f64 * t71472 - 0.41678e0_f64 * t71474 + 0.20839e0_f64 * t71477 - 0.104195e0_f64 * t71480 - 0.104195e0_f64 * t71483 + 0.62517e0_f64 * t71486 + 0.62517e0_f64 * t71489 + 0.309885e1_f64 * t71191 - 0.61977e1_f64 * t71195 - 0.123954e2_f64 * t71199;
    (t71955, t71968)
}
