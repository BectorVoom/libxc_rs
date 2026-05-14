//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 843/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk843<F: Float>(t1971: F, t3351: F, t3924: F, t6397: F, t39208: F, t9147: F, t2868: F, t8821: F, t8365: F, t8672: F, t26857: F, t8545: F, t26346: F, t8548: F, t5055: F, t8551: F) -> (F, F, F, F, F, F, F) {
    let t45909 = t3351 * t1971 * t3924 * t6397;
    let t45911 = t39208 * t9147;
    let t45914 = t2868 * t8821;
    let t45916 = t8365 * t8672;
    let t45918 = t26857 * t8545;
    let t45920 = t26346 * t8548;
    let t45922 = t5055 * t8551;
    (t45909, t45911, t45914, t45916, t45918, t45920, t45922)
}
