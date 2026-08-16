//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1038/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1038<F: Float>(t34944: F, t40888: F, t22: F, t235: F, t26115: F, t40902: F, t40921: F, t8630: F, t36978: F, t40894: F, t40898: F, t7198: F) -> (F, F, F, F, F) {
    let t41631 = t34944 * t40888;
    let t41634 = t235 * t26115 * t22;
    let t41635 = t41634 * t40902;
    let t41637 = t8630 * t40921;
    let t41639 = t36978 * t40894;
    let t41641 = t7198 * t40898;
    (t41631, t41635, t41637, t41639, t41641)
}
