//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 643/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk643<F: Float>(t13651: F, t833: F, t13056: F, t13059: F, t11627: F, t935: F, t1445: F, t2949: F, t3431: F, t13053: F, t13634: F, t13636: F, t13639: F, t13643: F, t13646: F, t13650: F, t813: F) -> (F, F, F, F, F) {
    let t13653 = 0.11502877786176224903e2 * t833 * t13651;
    let t13655 = 0.23005755572352449806e1 * t13056;
    let t13656 = 0.15337170381568299871e1 * t13059;
    let t13657 = t11627 * t935;
    let t13658 = t1445 * t13657;
    let t13660 = 0.43710935587469654631e2 * t833 * t13658;
    let t13661 = t2949 * t3431;
    let t13662 = t1445 * t13661;
    let t13665 = t13634 - 0.57514388930881124515e0 * t13636 + 0.95857314884801874192e0 * t13639 - t13643 - t13646 - t13650 + t13653 + 0.38342925953920749677e1 * t13053 - t13655 - t13656 + t13660 - 0.92023022289409799224e1 * t813 * t13662;
    (t13657, t13658, t13661, t13662, t13665)
}
