//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 564/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk564<F: Float>(t22: F, t3839: F, t262: F, t7617: F, t2103: F, t3826: F, t2115: F, t7638: F, t3819: F, t2118: F, t344: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t7641 = t3839 * t22;
    let t7645 = t262 * t7617;
    let t7646 = t2103 * t7645;
    let t7648 = t3826 * t22;
    let t7651 = t2115 * t7638;
    let t7653 = t3819 * t22;
    let t7656 = t2118 * t7645;
    let t7662 = t344 * t830;
    (t7641, t7645, t7646, t7648, t7651, t7653, t7656, t7662)
}
