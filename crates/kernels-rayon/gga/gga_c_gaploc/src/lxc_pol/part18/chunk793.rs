//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 793/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk793(t4820: f64, t7292: f64, t107: f64, t7284: f64, t787: f64, t1998: f64, t2004: f64, t2009: f64, t2034: f64, t2087: f64, t2178: f64, t2194: f64, t2625: f64, t2635: f64, t2642: f64, t5666: f64, t5748: f64, t7465: f64, t7468: f64, t7473: f64, t7476: f64, t7479: f64, t7482: f64, t7488: f64, t7493: f64, t7496: f64, t7500: f64, t7504: f64, t7506: f64, t7509: f64, t7513: f64, t807: f64, t813: f64, t833: f64) -> (f64, f64) {
    let t7514 = t4820 * t7292;
    let t7517 = t7284 * t107;
    let t7518 = t787 * t7517;
    let t7523 = -0.23005755572352449806e1_f64 * t1998 * t7465 - 0.71500979903700853338e0_f64 * t7468 * t2009 + 0.46011511144704899612e1_f64 * t2178 * t2642 + 0.46011511144704899612e1_f64 * t807 * t7473 + 0.23005755572352449806e1_f64 * t807 * t7476 - 0.61348681526273199482e1_f64 * t1998 * t7479 + 0.47667319935800568892e0_f64 * t2004 * t7482 - 0.92023022289409799224e1_f64 * t2194 * t2635 - 0.92023022289409799224e1_f64 * t813 * t7488 + 0.27606906686822939767e2_f64 * t5748 * t7493 - 0.62115540045351614476e2_f64 * t2087 * t7496 + 0.43710935587469654631e2_f64 * t833 * t7500 + 0.89376224879626066674e-1_f64 * t7504 - 0.18404604457881959845e2_f64 * t2087 * t7506 + 0.30674340763136599741e2_f64 * t833 * t7509 - 0.15889106645266856297e0_f64 * t7513 * t7514 + 0.23833659967900284446e0_f64 * t7518 * t2034 + 0.51123901271894332905e0_f64 * t5666 * t2625;
    (t7517, t7523)
}
