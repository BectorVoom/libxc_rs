//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1045/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1045<F: Float>(t11837: F, t1584: F, t26307: F, t3308: F, t574: F, t3309: F, t7566: F, t10725: F, t2651: F, t37754: F, t546: F, t39841: F, t6087: F, t10752: F, t30370: F, t38145: F, t6085: F, t7922: F) -> (F, F, F, F, F, F, F) {
    let t40024 = t1584 * t11837;
    let t40027 = t574 * t3308 * t26307;
    let t40029 = t7566 * t3309;
    let t40031 = t2651 * t10725;
    let t40033 = t546 * t37754;
    let t40035 = t40033 * t39841 * t6087;
    let t40038 = t30370 * t10752;
    let t40041 = t6085 * t38145 * t7922;
    (t40024, t40027, t40029, t40031, t40035, t40038, t40041)
}
