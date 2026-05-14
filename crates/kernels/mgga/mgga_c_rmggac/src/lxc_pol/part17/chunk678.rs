//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 678/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk678<F: Float>(t2139: F, t27: F, t3118: F, t333: F, t14366: F, t684: F, t2145: F, t352: F, t325: F, t4616: F, t235: F, t3807: F, t511: F, t2189: F, t7228: F, t3350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34803 = t2139 * t27 * t3118 * t333;
    let t34805 = t27 * t14366;
    let t34806 = t684 * t34805;
    let t34807 = 0.15556658869458454171e0 * t34806;
    let t34810 = t2145 * t27 * t3118 * t352;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    (t34803, t34805, t34807, t34810, t34812, t34813, t34828, t34846, t34847)
}
