//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 791/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk791<F: Float>(t4820: F, t7292: F, t107: F, t7284: F, t787: F, t1998: F, t2004: F, t2009: F, t2034: F, t2087: F, t2178: F, t2194: F, t2625: F, t2635: F, t2642: F, t5666: F, t5748: F, t7465: F, t7468: F, t7473: F, t7476: F, t7479: F, t7482: F, t7488: F, t7493: F, t7496: F, t7500: F, t7504: F, t7506: F, t7509: F, t7513: F, t807: F, t813: F, t833: F) -> (F, F) {
    let t7514 = t4820 * t7292;
    let t7517 = t7284 * t107;
    let t7518 = t787 * t7517;
    let t7523 = -F::cast_from(0.23005755572352449806e1_f64) * t1998 * t7465 - F::cast_from(0.71500979903700853338e0_f64) * t7468 * t2009 + F::cast_from(0.46011511144704899612e1_f64) * t2178 * t2642 + F::cast_from(0.46011511144704899612e1_f64) * t807 * t7473 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t7476 - F::cast_from(0.61348681526273199482e1_f64) * t1998 * t7479 + F::cast_from(0.47667319935800568892e0_f64) * t2004 * t7482 - F::cast_from(0.92023022289409799224e1_f64) * t2194 * t2635 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t7488 + F::cast_from(0.27606906686822939767e2_f64) * t5748 * t7493 - F::cast_from(0.62115540045351614476e2_f64) * t2087 * t7496 + F::cast_from(0.43710935587469654631e2_f64) * t833 * t7500 + F::cast_from(0.89376224879626066674e-1_f64) * t7504 - F::cast_from(0.18404604457881959845e2_f64) * t2087 * t7506 + F::cast_from(0.30674340763136599741e2_f64) * t833 * t7509 - F::cast_from(0.15889106645266856297e0_f64) * t7513 * t7514 + F::cast_from(0.23833659967900284446e0_f64) * t7518 * t2034 + F::cast_from(0.51123901271894332905e0_f64) * t5666 * t2625;
    (t7517, t7523)
}
