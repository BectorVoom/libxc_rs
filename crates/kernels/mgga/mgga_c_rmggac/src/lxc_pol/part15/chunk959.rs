//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 959/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk959<F: Float>(t39208: F, t9147: F, t2868: F, t8821: F, t8365: F, t8672: F, t26857: F, t8545: F, t26346: F, t8548: F, t5055: F, t8551: F) -> (F, F, F, F, F, F) {
    let t45911 = t39208 * t9147;
    let t45914 = t2868 * t8821;
    let t45916 = t8365 * t8672;
    let t45918 = t26857 * t8545;
    let t45920 = t26346 * t8548;
    let t45922 = t5055 * t8551;
    (t45911, t45914, t45916, t45918, t45920, t45922)
}
