//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 627/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk627<F: Float>(t11318: F, t874: F, t1445: F, t574: F, t12954: F, t12958: F, t13276: F, t1457: F, t4540: F, t2854: F, t3338: F, t11413: F, t4527: F, t11408: F, t1562: F, t3377: F, t3566: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13420 = t11318 * t874;
    let t13421 = t1445 * t13420;
    let t13423 = 0.92023022289409799224e1 * t574 * t13421;
    let t13424 = 0.23005755572352449806e1 * t12954;
    let t13425 = 0.15337170381568299871e1 * t12958;
    let t13426 = t1457 * t13276;
    let t13428 = 0.21450293971110256001e1 * t4540 * t13426;
    let t13429 = t2854 * t3338;
    let t13430 = t1445 * t13429;
    let t13433 = t11413 * t874;
    let t13434 = t1445 * t13433;
    let t13436 = 0.27606906686822939767e2 * t4527 * t13434;
    let t13437 = t11408 * t874;
    let t13438 = t1445 * t13437;
    let t13440 = 0.69017266717057349418e1 * t1562 * t13438;
    let t13442 = 0.25025342966295298669e1 * t3566 * t3377;
    (t13420, t13421, t13423, t13424, t13425, t13426, t13428, t13429, t13430, t13433, t13434, t13436, t13437, t13438, t13440, t13442)
}
