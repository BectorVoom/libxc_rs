//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 633/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk633<F: Float>(t13648: F, t813: F, t13555: F, t1445: F, t833: F, t13056: F, t13059: F, t11627: F, t935: F, t11757: F, t955: F, t11765: F, t13506: F, t1457: F, t6060: F, t13073: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13650 = 0.46011511144704899612e1 * t813 * t13648;
    let t13651 = t1445 * t13555;
    let t13653 = 0.11502877786176224903e2 * t833 * t13651;
    let t13655 = 0.23005755572352449806e1 * t13056;
    let t13656 = 0.15337170381568299871e1 * t13059;
    let t13657 = t11627 * t935;
    let t13658 = t1445 * t13657;
    let t13660 = 0.43710935587469654631e2 * t833 * t13658;
    let t13679 = 0.35750489951850426669e0 * t955 * t11757;
    let t13681 = 0.35750489951850426669e0 * t955 * t11765;
    let t13691 = t1457 * t13506;
    let t13693 = 0.21450293971110256001e1 * t6060 * t13691;
    let t13695 = 0.17875244975925213335e0 * t13073;
    (t13650, t13651, t13653, t13655, t13656, t13657, t13658, t13660, t13679, t13681, t13691, t13693, t13695)
}
