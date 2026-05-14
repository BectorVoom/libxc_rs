//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 810/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk810<F: Float>(t39208: F, t8457: F, t1907: F, t1971: F, t209: F, t236: F, t476: F, t7453: F, t2283: F, t38351: F, t39570: F, t8636: F, t39705: F, t8902: F, t17859: F, t9213: F) -> (F, F, F, F, F, F) {
    let t45277 = t39208 * t8457;
    let t45283 = t7453 * t1971 * t236 * t1907 * t476 * t209;
    let t45285 = t38351 * t2283;
    let t45289 = t39570 * t8636;
    let t45291 = t39705 * t8902;
    let t45293 = t17859 * t9213;
    (t45277, t45283, t45285, t45289, t45291, t45293)
}
