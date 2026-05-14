//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 748/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk748<F: Float>(t4820: F, t7292: F, t107: F, t7284: F, t787: F, t1998: F, t2004: F, t2009: F, t2034: F, t2087: F, t2178: F, t2194: F, t2625: F, t2635: F, t2642: F, t5666: F, t5748: F, t7465: F, t7468: F, t7473: F, t7476: F, t7479: F, t7482: F, t7488: F, t7493: F, t7496: F, t7500: F, t7504: F, t7506: F, t7509: F, t7513: F, t807: F, t813: F, t833: F) -> (F, F) {
    let t7514 = t4820 * t7292;
    let t7517 = t7284 * t107;
    let t7518 = t787 * t7517;
    let t7523 = -0.23005755572352449806e1 * t1998 * t7465 - 0.71500979903700853338e0 * t7468 * t2009 + 0.46011511144704899612e1 * t2178 * t2642 + 0.46011511144704899612e1 * t807 * t7473 + 0.23005755572352449806e1 * t807 * t7476 - 0.61348681526273199482e1 * t1998 * t7479 + 0.47667319935800568892e0 * t2004 * t7482 - 0.92023022289409799224e1 * t2194 * t2635 - 0.92023022289409799224e1 * t813 * t7488 + 0.27606906686822939767e2 * t5748 * t7493 - 0.62115540045351614476e2 * t2087 * t7496 + 0.43710935587469654631e2 * t833 * t7500 + 0.89376224879626066674e-1 * t7504 - 0.18404604457881959845e2 * t2087 * t7506 + 0.30674340763136599741e2 * t833 * t7509 - 0.15889106645266856297e0 * t7513 * t7514 + 0.23833659967900284446e0 * t7518 * t2034 + 0.51123901271894332905e0 * t5666 * t2625;
    (t7517, t7523)
}
