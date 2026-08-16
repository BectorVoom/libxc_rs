//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 721/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk721<F: Float>(t13644: F, t2087: F, t11622: F, t935: F, t1445: F, t813: F, t13555: F, t833: F, t13056: F, t13059: F, t11627: F, t2949: F, t3431: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13646 = F::cast_from(0.62115540045351614476e2_f64) * t2087 * t13644;
    let t13647 = t11622 * t935;
    let t13648 = t1445 * t13647;
    let t13650 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t13648;
    let t13651 = t1445 * t13555;
    let t13653 = F::cast_from(0.11502877786176224903e2_f64) * t833 * t13651;
    let t13655 = F::cast_from(0.23005755572352449806e1_f64) * t13056;
    let t13656 = F::cast_from(0.15337170381568299871e1_f64) * t13059;
    let t13657 = t11627 * t935;
    let t13658 = t1445 * t13657;
    let t13660 = F::cast_from(0.43710935587469654631e2_f64) * t833 * t13658;
    let t13661 = t2949 * t3431;
    (t13646, t13647, t13648, t13650, t13651, t13653, t13655, t13656, t13657, t13658, t13660, t13661)
}
