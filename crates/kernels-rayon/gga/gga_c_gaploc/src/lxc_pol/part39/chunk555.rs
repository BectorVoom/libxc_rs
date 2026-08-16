//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 555/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk555(t2464: f64, t9547: f64, t587: f64, t1441: f64, t1450: f64, t1537: f64, t1562: f64, t1572: f64, t557: f64, t574: f64, t597: f64, t9511: f64, t9514: f64, t9517: f64, t9520: f64, t9524: f64, t9528: f64, t9531: f64, t9534: f64, t9540: f64, t9541: f64, t9546: f64) -> (f64, f64, f64) {
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9550 = 0.85206502119823888169e-1_f64 * t9549;
    let t9551 = 0.15337170381568299871e2_f64 * t597 * t9511 - 0.61348681526273199483e1_f64 * t574 * t9514 - 0.23833659967900284446e0_f64 * t557 * t9517 + 0.47667319935800568892e0_f64 * t1572 * t9520 - 0.13803453343411469884e2_f64 * t1562 * t9524 - 0.23005755572352449806e1_f64 * t1450 * t9528 - 0.25561950635947166451e1_f64 * t1537 * t9531 + 0.51123901271894332902e0_f64 * t1441 * t9534 - t9540 + 0.1022478025437886658e1_f64 * t1441 * t9541 + t9546 + t9550;
    (t9549, t9550, t9551)
}
