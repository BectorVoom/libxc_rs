//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 705/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk705(t13657: f64, t1445: f64, t833: f64, t11757: f64, t955: f64, t11765: f64, t13506: f64, t1457: f64, t6060: f64, t13073: f64, t13078: f64, t13119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13658 = t1445 * t13657;
    let t13660 = 0.43710935587469654631e2_f64 * t833 * t13658;
    let t13679 = 0.35750489951850426669e0_f64 * t955 * t11757;
    let t13681 = 0.35750489951850426669e0_f64 * t955 * t11765;
    let t13691 = t1457 * t13506;
    let t13693 = 0.21450293971110256001e1_f64 * t6060 * t13691;
    let t13695 = 0.17875244975925213335e0_f64 * t13073;
    let t13697 = 0.59584149919750711116e-1_f64 * t13078;
    let t13700 = 0.11916829983950142223e0_f64 * t13119;
    (t13658, t13660, t13679, t13681, t13691, t13693, t13695, t13697, t13700)
}
