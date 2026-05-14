//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 493/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk493<F: Float>(t9539: F, t3125: F, t590: F, t2478: F, t888: F, t6576: F, t2334: F, t2465: F, t2464: F, t587: F, t1441: F, t1450: F, t1537: F, t1562: F, t1572: F, t557: F, t574: F, t597: F, t9511: F, t9514: F, t9517: F, t9520: F, t9524: F, t9528: F, t9531: F, t9534: F) -> (F, F, F) {
    let t9540 = 0.38342925953920749676e0 * t9539;
    let t9541 = t3125 * t590;
    let t9544 = t888 * t2478;
    let t9545 = t6576 * t9544;
    let t9546 = 0.38342925953920749676e0 * t9545;
    let t9547 = t2465 * t2334;
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9550 = 0.85206502119823888169e-1 * t9549;
    let t9551 = 0.15337170381568299871e2 * t597 * t9511 - 0.61348681526273199483e1 * t574 * t9514 - 0.23833659967900284446e0 * t557 * t9517 + 0.47667319935800568892e0 * t1572 * t9520 - 0.13803453343411469884e2 * t1562 * t9524 - 0.23005755572352449806e1 * t1450 * t9528 - 0.25561950635947166451e1 * t1537 * t9531 + 0.51123901271894332902e0 * t1441 * t9534 - t9540 + 0.1022478025437886658e1 * t1441 * t9541 + t9546 + t9550;
    (t9545, t9549, t9551)
}
