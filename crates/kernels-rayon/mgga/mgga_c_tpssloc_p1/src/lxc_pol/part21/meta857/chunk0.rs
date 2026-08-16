//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3108/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3108(t43780: f64, t43782: f64, t43816: f64, t43895: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t51039: f64, t51041: f64, t64028: f64, t64031: f64, t64033: f64, t64042: f64, t64045: f64) -> f64 {
    let t64389 = 0.16504875e0_f64 * t64028 - 0.16504875e0_f64 * t64031 + 0.776775e1_f64 * t64033 + 0.73586666666666666666e-1_f64 * t50968 + 0.36793333333333333333e-1_f64 * t50970 + 0.22076e0_f64 * t50972 + t43895 - 0.49057777777777777777e-1_f64 * t50978 + 0.13418888888888888889e0_f64 * t43780 + 0.26837777777777777778e0_f64 * t43782 - 0.62621481481481481482e0_f64 * t43816 - 0.258925e1_f64 * t64042 + 0.82785e-1_f64 * t64045 + 0.73586666666666666667e0_f64 * t51039 - 0.22076e0_f64 * t51041;
    t64389
}
