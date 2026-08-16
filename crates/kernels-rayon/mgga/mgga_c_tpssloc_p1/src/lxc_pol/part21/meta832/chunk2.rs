//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2934/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934(t10254: f64, t17691: f64, t41831: f64, t41863: f64, t41870: f64, t41872: f64, t48087: f64, t48096: f64, t48098: f64, t48103: f64, t48116: f64, t60091: f64, t60153: f64, t60156: f64) -> (f64, f64) {
    let t61103 = t10254 * t17691;
    let t61124 = -4.0_f64 / 3.0_f64 * t48087 - 10.0_f64 / 27.0_f64 * t41831 + 20.0_f64 / 27.0_f64 * t48096 - 2.0_f64 / 9.0_f64 * t48098 - 80.0_f64 / 81.0_f64 * t48103 - 80.0_f64 / 81.0_f64 * t41863 + 5.0_f64 / 27.0_f64 * t41870 + 5.0_f64 / 81.0_f64 * t41872 - 8.0_f64 / 81.0_f64 * t48116 + 4.0_f64 * t60091 - 4.0_f64 / 3.0_f64 * t60153 + 8.0_f64 / 27.0_f64 * t60156;
    (t61103, t61124)
}
