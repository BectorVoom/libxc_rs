//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 555/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk555<F: Float>(t2464: F, t9547: F, t587: F, t1441: F, t1450: F, t1537: F, t1562: F, t1572: F, t557: F, t574: F, t597: F, t9511: F, t9514: F, t9517: F, t9520: F, t9524: F, t9528: F, t9531: F, t9534: F, t9540: F, t9541: F, t9546: F) -> (F, F, F) {
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9550 = F::new(0.85206502119823888169e-1) * t9549;
    let t9551 = F::new(0.15337170381568299871e2) * t597 * t9511 - F::new(0.61348681526273199483e1) * t574 * t9514 - F::new(0.23833659967900284446e0) * t557 * t9517 + F::new(0.47667319935800568892e0) * t1572 * t9520 - F::new(0.13803453343411469884e2) * t1562 * t9524 - F::new(0.23005755572352449806e1) * t1450 * t9528 - F::new(0.25561950635947166451e1) * t1537 * t9531 + F::new(0.51123901271894332902e0) * t1441 * t9534 - t9540 + F::new(0.1022478025437886658e1) * t1441 * t9541 + t9546 + t9550;
    (t9549, t9550, t9551)
}
