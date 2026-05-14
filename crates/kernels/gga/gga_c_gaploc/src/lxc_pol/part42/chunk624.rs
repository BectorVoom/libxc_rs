//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 624/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk624<F: Float>(t12958: F, t13276: F, t1457: F, t4540: F, t11413: F, t874: F, t1445: F, t4527: F, t11408: F, t1562: F, t3377: F, t3566: F, t11362: F, t12969: F, t13397: F, t912: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13425 = 0.15337170381568299871e1 * t12958;
    let t13426 = t1457 * t13276;
    let t13428 = 0.21450293971110256001e1 * t4540 * t13426;
    let t13433 = t11413 * t874;
    let t13434 = t1445 * t13433;
    let t13436 = 0.27606906686822939767e2 * t4527 * t13434;
    let t13437 = t11408 * t874;
    let t13438 = t1445 * t13437;
    let t13440 = 0.69017266717057349418e1 * t1562 * t13438;
    let t13442 = 0.25025342966295298669e1 * t3566 * t3377;
    let t13444 = 0.10725146985555128001e1 * t11362 * t3377;
    let t13463 = 0.17875244975925213335e0 * t12969;
    let t13465 = t912 * t13397;
    (t13425, t13426, t13428, t13433, t13434, t13436, t13437, t13438, t13440, t13442, t13444, t13463, t13465)
}
