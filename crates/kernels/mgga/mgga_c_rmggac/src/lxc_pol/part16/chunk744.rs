//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 744/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk744<F: Float>(t14366: F, t27: F, t684: F, t2145: F, t3118: F, t352: F, t325: F, t4616: F, t235: F, t3807: F, t511: F, t2189: F, t7228: F) -> (F, F, F, F, F, F, F) {
    let t34805 = t27 * t14366;
    let t34806 = t684 * t34805;
    let t34810 = t2145 * t27 * t3118 * t352;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    let t34846 = t2189 * t7228;
    (t34805, t34806, t34810, t34812, t34813, t34828, t34846)
}
