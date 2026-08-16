//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 824/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk824<F: Float>(t1243: F, t3351: F, t511: F, t558: F, t7231: F, t17859: F, t7251: F, t7738: F, t7376: F, t7746: F, t1987: F, t38472: F) -> (F, F, F, F, F, F) {
    let t38483 = t3351 * t7231 * t511 * t558 * t1243;
    let t38485 = t17859 * t7251;
    let t38487 = t17859 * t7738;
    let t38489 = t17859 * t7376;
    let t38491 = t17859 * t7746;
    let t38493 = t38472 * t1987;
    (t38483, t38485, t38487, t38489, t38491, t38493)
}
