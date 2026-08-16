//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 724/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk724(t13538: f64, t1457: f64, t13506: f64, t6060: f64, t13073: f64, t1036: f64, t11001: f64, t13066: f64, t13070: f64, t13669: f64, t13673: f64, t13679: f64, t13681: f64, t13682: f64, t13685: f64, t2103: f64, t3025: f64, t317: f64, t833: f64) -> (f64, f64, f64) {
    let t13688 = t1457 * t13538;
    let t13691 = t1457 * t13506;
    let t13693 = 0.21450293971110256001e1_f64 * t6060 * t13691;
    let t13695 = 0.17875244975925213335e0_f64 * t13073;
    let t13696 = -0.76685851907841499353e0_f64 * t13066 + 0.23005755572352449806e1_f64 * t833 * t13669 + 0.35750489951850426669e0_f64 * t13673 * t317 + 0.71500979903700853338e0_f64 * t1036 * t11001 + t13679 + t13681 - 0.21450293971110256002e1_f64 * t3025 * t13682 + 0.23005755572352449806e2_f64 * t833 * t13685 + 0.14300195980740170668e1_f64 * t2103 * t13688 - t13693 + 0.76685851907841499353e0_f64 * t13070 - t13695;
    (t13688, t13691, t13696)
}
